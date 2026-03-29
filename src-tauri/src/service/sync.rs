//! 跨 Profile 同步服务
//!
//! 通过 service 层读取源端数据、写入目标数据库，确保业务逻辑一致性。
//! 加密内容自动解密后重新加密，笔记创建自动生成历史记录。

use std::collections::HashMap;
use std::path::Path;

use chrono::Local;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect, TransactionTrait};
use tauri::{AppHandle, Emitter};
use tracing::info;

use crate::{
    config::database_connection_from_profile,
    i18n::t,
    entity,
    model::{DataStats, OperateSource, SyncLog, SyncMode, SyncOptions, SyncPreview, SyncProgress},
    service::{backup, note, note_template, notebook, profile, settings, sync_log, tag},
};

/// ID 映射表，用于修复外键关系
struct IdMaps {
    notebook: HashMap<i64, i64>,
    tag: HashMap<i64, i64>,
    #[allow(dead_code)]
    note: HashMap<i64, i64>,
}

impl IdMaps {
    fn new() -> Self {
        Self {
            notebook: HashMap::new(),
            tag: HashMap::new(),
            note: HashMap::new(),
        }
    }
}

/// 发送进度事件到前端
fn emit_progress(
    app_handle: &AppHandle,
    log_id: i64,
    stage: &str,
    table_name: &str,
    current: u32,
    total: u32,
    record_name: &str,
) {
    let _ = app_handle.emit(
        "sync-progress",
        SyncProgress {
            sync_log_id: log_id,
            stage: stage.to_string(),
            table_name: table_name.to_string(),
            current,
            total,
            record_name: record_name.to_string(),
        },
    );
}

/// 统计数据库中的数据量
async fn count_stats(
    db: &DatabaseConnection,
    profile_name: &str,
    db_type: &str,
) -> anyhow::Result<DataStats> {
    Ok(DataStats {
        profile_name: profile_name.to_string(),
        db_type: db_type.to_string(),
        notebook_count: entity::notebook::Entity::find().count(db).await? as u32,
        tag_count: entity::tag::Entity::find().count(db).await? as u32,
        note_count: entity::note::Entity::find().count(db).await? as u32,
        note_history_count: entity::note_history::Entity::find().count(db).await? as u32,
        template_count: entity::note_template::Entity::find().count(db).await? as u32,
        settings_count: entity::settings::Entity::find().count(db).await? as u32,
    })
}

/// 连接目标 Profile 数据库
async fn connect_target(
    app_data_dir: &Path,
    target_profile_id: &str,
    target_db_password: Option<&str>,
) -> anyhow::Result<DatabaseConnection> {
    let config = profile::read_profile(app_data_dir, target_profile_id)?;
    let db = database_connection_from_profile(&config, target_db_password).await?;
    // 运行迁移确保目标数据库表结构最新
    use crate::migration::Migrator;
    use sea_orm_migration::MigratorTrait;
    Migrator::up(&db, None).await?;
    Ok(db)
}

/// 获取同步预览信息
pub async fn get_preview(
    source_db: &DatabaseConnection,
    app_data_dir: &Path,
    source_profile_id: &str,
    target_profile_id: &str,
    target_db_password: Option<&str>,
) -> anyhow::Result<SyncPreview> {
    let source_config = profile::read_profile(app_data_dir, source_profile_id)?;
    let target_config = profile::read_profile(app_data_dir, target_profile_id)?;

    let source_stats = count_stats(
        source_db,
        &source_config.name,
        &source_config.datasource.db_type,
    )
    .await?;

    let target_db = connect_target(app_data_dir, target_profile_id, target_db_password).await?;
    let target_stats = count_stats(
        &target_db,
        &target_config.name,
        &target_config.datasource.db_type,
    )
    .await?;
    target_db.close().await?;

    Ok(SyncPreview {
        source: source_stats,
        target: target_stats,
    })
}

/// 执行同步核心流程
pub async fn sync_to_profile(
    source_db: &DatabaseConnection,
    app_data_dir: &Path,
    source_profile_id: &str,
    options: &SyncOptions,
    target_db_password: Option<&str>,
    source_encryption_key: Option<&str>,
    target_encryption_key: Option<&str>,
    app_handle: &AppHandle,
) -> anyhow::Result<SyncLog> {
    let started_at = Local::now().naive_local();

    let source_config = profile::read_profile(app_data_dir, source_profile_id)?;
    let target_config = profile::read_profile(app_data_dir, &options.target_profile_id)?;

    let mode_str = match options.mode {
        SyncMode::Append => "append",
        SyncMode::Overwrite => "overwrite",
    };

    // ═══════════════════════════════════════
    // 1. 在源端创建 sync_log 记录
    // ═══════════════════════════════════════
    let log = sync_log::create(
        source_db,
        &SyncLog {
            source_profile: source_profile_id.to_string(),
            target_profile: options.target_profile_id.clone(),
            source_db_type: source_config.datasource.db_type.clone(),
            target_db_type: target_config.datasource.db_type.clone(),
            sync_mode: mode_str.to_string(),
            sync_scope: serde_json::to_string(&options.scope)?,
            backup_format: options.backup_format.clone(),
            status: "running".to_string(),
            started_at: Some(started_at),
            ..Default::default()
        },
    )
    .await?;
    let log_id = log.id;

    info!(
        "Starting sync: {} -> {}, mode: {}",
        source_profile_id, options.target_profile_id, mode_str
    );

    // ═══════════════════════════════════════
    // 2. 自动备份
    // ═══════════════════════════════════════
    if options.auto_backup {
        let format = options.backup_format.as_deref().unwrap_or("sql");
        let ext = match format {
            "excel" => "xlsx",
            "csv" => "csv",
            _ => "sql",
        };
        let ts = Local::now().format("%Y%m%d_%H%M%S");
        let backup_dir = app_data_dir.join("backups");
        if !backup_dir.exists() {
            std::fs::create_dir_all(&backup_dir)?;
        }

        // 备份源端
        let source_file = format!("pre_sync_source_{}_{}.{}", source_profile_id, ts, ext);
        let source_path = backup_dir.join(&source_file);
        let source_path_str = source_path.to_string_lossy().to_string();
        match format {
            "excel" => backup::export_excel(source_db, &source_path_str).await?,
            "csv" => backup::export_csv(source_db, &source_path_str).await?,
            _ => backup::export_sql(source_db, &source_path_str).await?,
        }
        sync_log::update_backup(source_db, log_id, Some(&source_file), None).await?;
        emit_progress(app_handle, log_id, "backup", "source", 1, 2, &source_file);

        // 备份目标端
        let target_db =
            connect_target(app_data_dir, &options.target_profile_id, target_db_password).await?;
        let target_file = format!(
            "pre_sync_target_{}_{}.{}",
            options.target_profile_id, ts, ext
        );
        let target_path = backup_dir.join(&target_file);
        let target_path_str = target_path.to_string_lossy().to_string();
        match format {
            "excel" => backup::export_excel(&target_db, &target_path_str).await?,
            "csv" => backup::export_csv(&target_db, &target_path_str).await?,
            _ => backup::export_sql(&target_db, &target_path_str).await?,
        }
        target_db.close().await?;
        sync_log::update_backup(source_db, log_id, None, Some(&target_file)).await?;
        emit_progress(app_handle, log_id, "backup", "target", 2, 2, &target_file);
    }

    // ═══════════════════════════════════════
    // 3. 连接目标数据库
    // ═══════════════════════════════════════
    let target_db =
        connect_target(app_data_dir, &options.target_profile_id, target_db_password).await?;

    // Overwrite 模式：清空目标表（使用 backup 已有的 clear 逻辑）
    if matches!(options.mode, SyncMode::Overwrite) {
        let txn = target_db.begin().await?;
        // 按外键反序清空
        entity::note_tags::Entity::delete_many().exec(&txn).await?;
        entity::note_history::Entity::delete_many()
            .exec(&txn)
            .await?;
        entity::note::Entity::delete_many().exec(&txn).await?;
        entity::tag::Entity::delete_many().exec(&txn).await?;
        entity::notebook::Entity::delete_many().exec(&txn).await?;
        if options.scope.templates {
            entity::note_template::Entity::delete_many()
                .exec(&txn)
                .await?;
        }
        if options.scope.settings {
            entity::settings::Entity::delete_many().exec(&txn).await?;
        }
        txn.commit().await?;
        emit_progress(
            app_handle,
            log_id,
            "clear",
            "all",
            1,
            1,
            &t("sync.progress.clearTarget", &[]),
        );
    }

    // ═══════════════════════════════════════
    // 4. 按依赖顺序同步
    // ═══════════════════════════════════════
    let mut id_maps = IdMaps::new();
    let mut total: i32 = 0;
    let mut success: i32 = 0;
    let mut failed: i32 = 0;

    // 4a. 同步笔记本
    if options.scope.notebooks {
        let source_notebooks = notebook::find_all(source_db).await?;
        let count = source_notebooks.len() as u32;

        // 按 parent_id 排序确保父级先插入（parent_id=0 的排前面）
        let mut sorted_notebooks = source_notebooks;
        sorted_notebooks.sort_by_key(|nb| nb.parent_id);

        for (i, nb) in sorted_notebooks.iter().enumerate() {
            let source_id = nb.id;
            let synced_at = Local::now().naive_local();

            let mut new_nb = nb.clone();
            new_nb.id = 0;
            new_nb.parent_id = id_maps.notebook.get(&nb.parent_id).copied().unwrap_or(0);

            match notebook::create(&target_db, &new_nb).await {
                Ok(Some(created)) => {
                    id_maps.notebook.insert(source_id, created.id);
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "notebook",
                        source_id,
                        Some(created.id),
                        &nb.name,
                        "success",
                        None,
                        synced_at,
                    )
                    .await?;
                    success += 1;
                }
                Ok(None) => {
                    sync_log::add_detail(
                        source_db, log_id, "notebook", source_id, None, &nb.name, "skipped", None,
                        synced_at,
                    )
                    .await?;
                }
                Err(e) => {
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "notebook",
                        source_id,
                        None,
                        &nb.name,
                        "failed",
                        Some(&e.to_string()),
                        synced_at,
                    )
                    .await?;
                    failed += 1;
                }
            }
            total += 1;
            emit_progress(
                app_handle,
                log_id,
                "sync",
                "notebook",
                i as u32 + 1,
                count,
                &nb.name,
            );
        }
    }

    // 4b. 同步标签
    if options.scope.tags {
        let source_tags = tag::find_all(source_db).await?;
        let count = source_tags.len() as u32;

        for (i, t) in source_tags.iter().enumerate() {
            let source_id = t.id;
            let synced_at = Local::now().naive_local();

            let mut new_tag = t.clone();
            new_tag.id = 0;

            match tag::create(&target_db, &new_tag).await {
                Ok(Some(created)) => {
                    id_maps.tag.insert(source_id, created.id);
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "tag",
                        source_id,
                        Some(created.id),
                        &t.name,
                        "success",
                        None,
                        synced_at,
                    )
                    .await?;
                    success += 1;
                }
                Ok(None) => {
                    sync_log::add_detail(
                        source_db, log_id, "tag", source_id, None, &t.name, "skipped", None,
                        synced_at,
                    )
                    .await?;
                }
                Err(e) => {
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "tag",
                        source_id,
                        None,
                        &t.name,
                        "failed",
                        Some(&e.to_string()),
                        synced_at,
                    )
                    .await?;
                    failed += 1;
                }
            }
            total += 1;
            emit_progress(
                app_handle,
                log_id,
                "sync",
                "tag",
                i as u32 + 1,
                count,
                &t.name,
            );
        }
    }

    // 4c. 同步笔记（核心：通过 service 层保证加密 + 历史一致性）
    // 同步批量大小（防止大量数据一次性占用过多内存）
    const SYNC_BATCH_SIZE: u64 = 200;

    if options.scope.notes {
        let count = entity::note::Entity::find()
            .count(source_db)
            .await? as u32;
        let mut global_idx: u32 = 0;

        let total_batches = (count as u64 + SYNC_BATCH_SIZE - 1) / SYNC_BATCH_SIZE;
        for batch_idx in 0..total_batches {
            let source_notes: Vec<entity::note::Model> = entity::note::Entity::find()
                .offset(batch_idx * SYNC_BATCH_SIZE)
                .limit(SYNC_BATCH_SIZE)
                .all(source_db)
                .await?;

        for src_note in source_notes.iter() {
            let i = global_idx;
            global_idx += 1;
            let source_id = src_note.id;
            let synced_at = Local::now().naive_local();

            // 通过 service 层读取（自动解密 + 获取标签）
            let decrypted =
                note::find_by_id_with_key(source_db, source_id, source_encryption_key).await?;

            if let Some(mut n) = decrypted {
                // 重映射外键
                n.id = 0;
                n.notebook_id = id_maps
                    .notebook
                    .get(&n.notebook_id)
                    .copied()
                    .unwrap_or(n.notebook_id);
                // 重映射标签 ID
                for tag_item in &mut n.tags {
                    tag_item.id = id_maps
                        .tag
                        .get(&tag_item.id)
                        .copied()
                        .unwrap_or(tag_item.id);
                }

                // 保持原始的删除状态和置顶状态
                let original_deleted_at = src_note.deleted_at;
                let original_is_pinned = src_note.is_pinned;

                // 通过 service 层写入目标（自动加密 + 自动生成 history）
                match note::create_with_key(
                    &target_db,
                    &n,
                    OperateSource::Sync,
                    target_encryption_key,
                )
                .await
                {
                    Ok(Some(created)) => {
                        id_maps.note.insert(source_id, created.id);

                        // 恢复原始状态（create_with_key 会重置 is_pinned=0 和 deleted_at=None）
                        if original_deleted_at.is_some() || original_is_pinned != 0 {
                            use sea_orm::{ActiveModelTrait, ActiveValue::Set, IntoActiveModel};
                            if let Some(entity) = entity::note::Entity::find_by_id(created.id)
                                .one(&target_db)
                                .await?
                            {
                                let mut am: entity::note::ActiveModel = entity.into_active_model();
                                if original_deleted_at.is_some() {
                                    am.deleted_at = Set(original_deleted_at);
                                }
                                if original_is_pinned != 0 {
                                    am.is_pinned = Set(original_is_pinned);
                                }
                                am.update(&target_db).await?;
                            }
                        }

                        sync_log::add_detail(
                            source_db,
                            log_id,
                            "note",
                            source_id,
                            Some(created.id),
                            &n.title,
                            "success",
                            None,
                            synced_at,
                        )
                        .await?;
                        success += 1;
                    }
                    Ok(None) => {
                        sync_log::add_detail(
                            source_db, log_id, "note", source_id, None, &n.title, "skipped", None,
                            synced_at,
                        )
                        .await?;
                    }
                    Err(e) => {
                        sync_log::add_detail(
                            source_db,
                            log_id,
                            "note",
                            source_id,
                            None,
                            &n.title,
                            "failed",
                            Some(&e.to_string()),
                            synced_at,
                        )
                        .await?;
                        failed += 1;
                    }
                }
            } else {
                sync_log::add_detail(
                    source_db,
                    log_id,
                    "note",
                    source_id,
                    None,
                    &src_note.title,
                    "skipped",
                    Some(&t("sync.progress.sourceNoteNotFound", &[])),
                    synced_at,
                )
                .await?;
            }
            total += 1;
            emit_progress(
                app_handle,
                log_id,
                "sync",
                "note",
                i + 1,
                count,
                &src_note.title,
            );
        }
        } // end batch loop
    }

    // 4d. 同步笔记历史（可选：由于 create_with_key 已自动生成了同步的历史记录，
    //     这里同步的是源端的原始历史记录）
    if options.scope.note_histories {
        let count = entity::note_history::Entity::find()
            .count(source_db)
            .await? as u32;
        let mut global_hist_idx: u32 = 0;

        let total_hist_batches = (count as u64 + SYNC_BATCH_SIZE - 1) / SYNC_BATCH_SIZE;
        for batch_idx in 0..total_hist_batches {
            let source_histories: Vec<entity::note_history::Model> =
                entity::note_history::Entity::find()
                    .offset(batch_idx * SYNC_BATCH_SIZE)
                    .limit(SYNC_BATCH_SIZE)
                    .all(source_db)
                    .await?;

        for h in source_histories.iter() {
            let i = global_hist_idx as usize;
            global_hist_idx += 1;
            let source_id = h.id;
            let synced_at = Local::now().naive_local();
            let note_id = id_maps.note.get(&h.note_id).copied().unwrap_or(h.note_id);

            use sea_orm::ActiveValue::{NotSet, Set};
            let active_model = entity::note_history::ActiveModel {
                id: NotSet,
                note_id: Set(note_id),
                old_content: Set(h.old_content.clone()),
                new_content: Set(h.new_content.clone()),
                extra: Set(h.extra.clone()),
                operate_type: Set(h.operate_type),
                operate_source: Set(h.operate_source),
                operate_time: Set(h.operate_time),
                create_time: Set(h.create_time),
            };

            use sea_orm::ActiveModelTrait;
            match active_model.insert(&target_db).await {
                Ok(inserted) => {
                    let name = format!("note_id={} type={}", h.note_id, h.operate_type);
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "note_history",
                        source_id,
                        Some(inserted.id),
                        &name,
                        "success",
                        None,
                        synced_at,
                    )
                    .await?;
                    success += 1;
                }
                Err(e) => {
                    let name = format!("note_id={} type={}", h.note_id, h.operate_type);
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "note_history",
                        source_id,
                        None,
                        &name,
                        "failed",
                        Some(&e.to_string()),
                        synced_at,
                    )
                    .await?;
                    failed += 1;
                }
            }
            total += 1;
            if (i + 1) % 50 == 0 || i + 1 == count as usize {
                emit_progress(
                    app_handle,
                    log_id,
                    "sync",
                    "note_history",
                    i as u32 + 1,
                    count,
                    &format!("{}/{}", i + 1, count),
                );
            }
        }
        } // end history batch loop
    }

    // 4e. 同步模板
    if options.scope.templates {
        let source_templates = note_template::find_all(source_db).await?;
        let count = source_templates.len() as u32;

        for (i, t) in source_templates.iter().enumerate() {
            let source_id = t.id;
            let synced_at = Local::now().naive_local();

            let mut new_t = t.clone();
            new_t.id = 0;

            match note_template::create(&target_db, &new_t).await {
                Ok(Some(created)) => {
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "note_template",
                        source_id,
                        Some(created.id),
                        &t.name,
                        "success",
                        None,
                        synced_at,
                    )
                    .await?;
                    success += 1;
                }
                Ok(None) => {
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "note_template",
                        source_id,
                        None,
                        &t.name,
                        "skipped",
                        None,
                        synced_at,
                    )
                    .await?;
                }
                Err(e) => {
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "note_template",
                        source_id,
                        None,
                        &t.name,
                        "failed",
                        Some(&e.to_string()),
                        synced_at,
                    )
                    .await?;
                    failed += 1;
                }
            }
            total += 1;
            emit_progress(
                app_handle,
                log_id,
                "sync",
                "note_template",
                i as u32 + 1,
                count,
                &t.name,
            );
        }
    }

    // 4f. 同步设置
    if options.scope.settings {
        let source_settings = settings::get_all(source_db).await?;
        if !source_settings.is_empty() {
            let synced_at = Local::now().naive_local();
            let count = source_settings.len();

            match settings::save(&target_db, source_settings).await {
                Ok(_) => {
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "settings",
                        0,
                        None,
                        &t("sync.progress.settingsCount", &[&count.to_string()]),
                        "success",
                        None,
                        synced_at,
                    )
                    .await?;
                    success += 1;
                }
                Err(e) => {
                    sync_log::add_detail(
                        source_db,
                        log_id,
                        "settings",
                        0,
                        None,
                        &t("sync.progress.settingsCount", &[&count.to_string()]),
                        "failed",
                        Some(&e.to_string()),
                        synced_at,
                    )
                    .await?;
                    failed += 1;
                }
            }
            total += 1;
            emit_progress(
                app_handle,
                log_id,
                "sync",
                "settings",
                1,
                1,
                &t("sync.progress.settings", &[]),
            );
        }
    }

    // ═══════════════════════════════════════
    // 5. 关闭目标连接，更新日志
    // ═══════════════════════════════════════
    target_db.close().await?;

    let status = if failed > 0 { "completed" } else { "completed" };
    sync_log::finish(source_db, log_id, status, total, success, failed, None).await?;

    info!(
        "Sync completed: total={}, success={}, failed={}",
        total, success, failed
    );

    sync_log::find_by_id(source_db, log_id)
        .await?
        .ok_or_else(|| anyhow::Error::from(crate::error::AppError::code("SYNC_LOG_NOT_FOUND")))
}
