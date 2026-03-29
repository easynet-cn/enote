//! 同步日志服务模块
//!
//! 提供同步日志和明细的 CRUD 操作

use chrono::{Local, NaiveDateTime};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, TransactionTrait,
};

use crate::{
    entity,
    model::{PageParam, PageResult, SyncLog, SyncLogDetail},
};

/// 创建同步日志
pub async fn create(db: &DatabaseConnection, log: &SyncLog) -> anyhow::Result<SyncLog> {
    let now = Local::now().naive_local();

    let active_model = entity::sync_log::ActiveModel {
        id: NotSet,
        source_profile: Set(log.source_profile.clone()),
        target_profile: Set(log.target_profile.clone()),
        source_db_type: Set(log.source_db_type.clone()),
        target_db_type: Set(log.target_db_type.clone()),
        sync_mode: Set(log.sync_mode.clone()),
        sync_scope: Set(log.sync_scope.clone()),
        backup_format: Set(log.backup_format.clone()),
        source_backup: Set(log.source_backup.clone()),
        target_backup: Set(log.target_backup.clone()),
        status: Set(log.status.clone()),
        total_count: Set(0),
        success_count: Set(0),
        failed_count: Set(0),
        error_message: Set(None),
        started_at: Set(log.started_at.unwrap_or(now)),
        finished_at: Set(None),
        create_time: Set(now),
    };

    let entity = active_model.insert(db).await?;
    Ok(SyncLog::from(entity))
}

/// 根据 ID 查询同步日志
pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<Option<SyncLog>> {
    let result = entity::sync_log::Entity::find_by_id(id).one(db).await?;
    Ok(result.map(SyncLog::from))
}

/// 更新同步日志的备份信息
pub async fn update_backup(
    db: &DatabaseConnection,
    id: i64,
    source_backup: Option<&str>,
    target_backup: Option<&str>,
) -> anyhow::Result<()> {
    if let Some(entity) = entity::sync_log::Entity::find_by_id(id).one(db).await? {
        let mut active_model: entity::sync_log::ActiveModel = entity.into();
        if let Some(path) = source_backup {
            active_model.source_backup = Set(Some(path.to_string()));
        }
        if let Some(path) = target_backup {
            active_model.target_backup = Set(Some(path.to_string()));
        }
        active_model.update(db).await?;
    }
    Ok(())
}

/// 更新同步日志的计数
pub async fn update_counts(
    db: &DatabaseConnection,
    id: i64,
    total: i32,
    success: i32,
    failed: i32,
) -> anyhow::Result<()> {
    if let Some(entity) = entity::sync_log::Entity::find_by_id(id).one(db).await? {
        let mut active_model: entity::sync_log::ActiveModel = entity.into();
        active_model.total_count = Set(total);
        active_model.success_count = Set(success);
        active_model.failed_count = Set(failed);
        active_model.update(db).await?;
    }
    Ok(())
}

/// 完成同步日志
pub async fn finish(
    db: &DatabaseConnection,
    id: i64,
    status: &str,
    total: i32,
    success: i32,
    failed: i32,
    error_message: Option<&str>,
) -> anyhow::Result<()> {
    let now = Local::now().naive_local();
    if let Some(entity) = entity::sync_log::Entity::find_by_id(id).one(db).await? {
        let mut active_model: entity::sync_log::ActiveModel = entity.into();
        active_model.status = Set(status.to_string());
        active_model.total_count = Set(total);
        active_model.success_count = Set(success);
        active_model.failed_count = Set(failed);
        active_model.finished_at = Set(Some(now));
        if let Some(msg) = error_message {
            active_model.error_message = Set(Some(msg.to_string()));
        }
        active_model.update(db).await?;
    }
    Ok(())
}

/// 添加同步明细记录
#[allow(clippy::too_many_arguments)]
pub async fn add_detail(
    db: &DatabaseConnection,
    sync_log_id: i64,
    table_name: &str,
    source_id: i64,
    target_id: Option<i64>,
    record_name: &str,
    status: &str,
    error_message: Option<&str>,
    synced_at: NaiveDateTime,
) -> anyhow::Result<SyncLogDetail> {
    let now = Local::now().naive_local();

    let active_model = entity::sync_log_detail::ActiveModel {
        id: NotSet,
        sync_log_id: Set(sync_log_id),
        table_name: Set(table_name.to_string()),
        source_id: Set(source_id),
        target_id: Set(target_id),
        record_name: Set(record_name.to_string()),
        status: Set(status.to_string()),
        error_message: Set(error_message.map(|s| s.to_string())),
        synced_at: Set(synced_at),
        create_time: Set(now),
    };

    let entity = active_model.insert(db).await?;
    Ok(SyncLogDetail::from(entity))
}

/// 分页查询同步日志
pub async fn search_page(
    db: &DatabaseConnection,
    page: &PageParam,
) -> anyhow::Result<PageResult<SyncLog>> {
    let total = entity::sync_log::Entity::find().count(db).await? as i64;

    let data = entity::sync_log::Entity::find()
        .order_by_desc(entity::sync_log::Column::Id)
        .offset(page.start() as u64)
        .limit(page.page_size as u64)
        .all(db)
        .await?
        .into_iter()
        .map(SyncLog::from)
        .collect();

    let mut result = PageResult::with_data(total, data);
    result.total_pages(page.page_size);
    Ok(result)
}

/// 分页查询同步明细（支持按表名和状态过滤）
pub async fn search_detail_page(
    db: &DatabaseConnection,
    sync_log_id: i64,
    table_name: Option<&str>,
    status: Option<&str>,
    page: &PageParam,
) -> anyhow::Result<PageResult<SyncLogDetail>> {
    let mut query = entity::sync_log_detail::Entity::find()
        .filter(entity::sync_log_detail::Column::SyncLogId.eq(sync_log_id));

    if let Some(tn) = table_name
        && !tn.is_empty() {
            query = query.filter(entity::sync_log_detail::Column::TableName.eq(tn));
        }
    if let Some(st) = status
        && !st.is_empty() {
            query = query.filter(entity::sync_log_detail::Column::Status.eq(st));
        }

    let total = query.clone().count(db).await? as i64;

    let data = query
        .order_by_asc(entity::sync_log_detail::Column::Id)
        .offset(page.start() as u64)
        .limit(page.page_size as u64)
        .all(db)
        .await?
        .into_iter()
        .map(SyncLogDetail::from)
        .collect();

    let mut result = PageResult::with_data(total, data);
    result.total_pages(page.page_size);
    Ok(result)
}

/// 删除同步日志（含明细）
pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    let txn = db.begin().await?;

    entity::sync_log_detail::Entity::delete_many()
        .filter(entity::sync_log_detail::Column::SyncLogId.eq(id))
        .exec(&txn)
        .await?;

    entity::sync_log::Entity::delete_by_id(id)
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(())
}

/// 清空所有同步日志
pub async fn clear_all(db: &DatabaseConnection) -> anyhow::Result<()> {
    let txn = db.begin().await?;

    entity::sync_log_detail::Entity::delete_many()
        .exec(&txn)
        .await?;

    entity::sync_log::Entity::delete_many().exec(&txn).await?;

    txn.commit().await?;
    Ok(())
}
