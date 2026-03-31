//! 修复 FTS5 触发器：delete 命令必须使用与 insert 相同的 clean_content
//!
//! ## 问题
//! FTS5 外部内容表（content='note'）要求 delete 命令传入的列值
//! 必须与索引时传入的值完全一致。之前的触发器在 delete 时传入
//! 原始 HTML content（old.content / n.content），但 insert 时传入
//! 的是 clean_content（去除 HTML 标签后的文本），导致 FTS5 内部
//! token 计数不一致，触发 "database disk image is malformed" 错误。
//!
//! ## 修复
//! 1. 所有触发器的 delete 操作统一使用 clean_content 处理 content
//! 2. 加密前缀从 ENOTE_ENC_V1 更新为 ENOTE_ENC_V2（匹配实际加密模块）
//! 3. PostgreSQL: 补充 note_attachment 表 id 列类型修复（m20260329 遗漏）
//!
//! ## 影响
//! - 修复后笔记的添加、编辑、删除操作恢复正常
//! - 重建 FTS 索引以修复已损坏的索引数据

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 构建获取笔记关联标签名称的子查询 SQL
const TAGS_SUBQUERY: &str = "(SELECT group_concat(t.name, ' ') FROM note_tags nt JOIN tag t ON t.id = nt.tag_id WHERE nt.note_id = {note_id})";

/// 构建去除 HTML 标签的 content 表达式（跳过加密内容）
/// 修复：加密前缀从 V1 更新为 V2，匹配 crypto.rs 中的 ENCRYPTED_PREFIX
const CLEAN_CONTENT_EXPR: &str = r#"
    CASE WHEN {content} LIKE 'ENOTE_ENC_V1:%' OR {content} LIKE 'ENOTE_ENC_V2:%' THEN ''
    ELSE replace(replace(replace(replace(replace(replace(
        {content},
        '<br>', ' '), '<br/>', ' '), '<br />', ' '),
        '</p>', ' '), '</div>', ' '), '</li>', ' ')
    END
"#;

fn tags_for(note_id: &str) -> String {
    TAGS_SUBQUERY.replace("{note_id}", note_id)
}

fn clean_content(content_ref: &str) -> String {
    CLEAN_CONTENT_EXPR.replace("{content}", content_ref)
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        if backend != sea_orm::DatabaseBackend::Sqlite {
            tracing::info!("跳过 FTS 触发器修复：当前数据库不是 SQLite");
            return Ok(());
        }

        // 删除旧的触发器
        for trigger in [
            "note_fts_insert",
            "note_fts_update",
            "note_fts_delete",
            "note_tags_fts_insert",
            "note_tags_fts_delete",
        ] {
            db.execute_unprepared(&format!("DROP TRIGGER IF EXISTS {};", trigger))
                .await?;
        }

        // 删除旧的 FTS 表并重建（修复已损坏的索引）
        db.execute_unprepared("DROP TABLE IF EXISTS note_fts;")
            .await?;

        db.execute_unprepared(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS note_fts USING fts5(
                title,
                content,
                tags,
                content='note',
                content_rowid='id',
                tokenize='trigram'
            );
            "#,
        )
        .await?;

        // 重新同步数据到 FTS 表
        let init_sql = format!(
            r#"
            INSERT INTO note_fts(rowid, title, content, tags)
            SELECT n.id, n.title,
                {},
                COALESCE({}, '')
            FROM note n WHERE n.deleted_at IS NULL;
            "#,
            clean_content("n.content"),
            tags_for("n.id"),
        );
        db.execute_unprepared(&init_sql).await?;

        // note 插入触发器（未变）
        let insert_trigger = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_insert AFTER INSERT ON note
            WHEN new.deleted_at IS NULL
            BEGIN
                INSERT INTO note_fts(rowid, title, content, tags)
                VALUES (new.id, new.title,
                    {},
                    COALESCE({}, '')
                );
            END;
            "#,
            clean_content("new.content"),
            tags_for("new.id"),
        );
        db.execute_unprepared(&insert_trigger).await?;

        // note 更新触发器（修复：delete 使用 clean_content）
        let update_trigger = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_update AFTER UPDATE ON note BEGIN
                INSERT INTO note_fts(note_fts, rowid, title, content, tags)
                VALUES ('delete', old.id, old.title,
                    {},
                    COALESCE({}, ''));
                INSERT OR IGNORE INTO note_fts(rowid, title, content, tags)
                SELECT new.id, new.title,
                    {},
                    COALESCE({}, '')
                WHERE new.deleted_at IS NULL;
            END;
            "#,
            clean_content("old.content"),
            tags_for("old.id"),
            clean_content("new.content"),
            tags_for("new.id"),
        );
        db.execute_unprepared(&update_trigger).await?;

        // note 删除触发器（修复：delete 使用 clean_content）
        let delete_trigger = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_delete AFTER DELETE ON note BEGIN
                INSERT INTO note_fts(note_fts, rowid, title, content, tags)
                VALUES ('delete', old.id, old.title,
                    {},
                    COALESCE({}, ''));
            END;
            "#,
            clean_content("old.content"),
            tags_for("old.id"),
        );
        db.execute_unprepared(&delete_trigger).await?;

        // note_tags 插入触发器（修复：delete 使用 clean_content）
        let note_tags_insert = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_tags_fts_insert AFTER INSERT ON note_tags
            BEGIN
                INSERT INTO note_fts(note_fts, rowid, title, content, tags)
                SELECT 'delete', n.id, n.title,
                    {},
                    COALESCE({}, '')
                FROM note n WHERE n.id = new.note_id AND n.deleted_at IS NULL;
                INSERT OR IGNORE INTO note_fts(rowid, title, content, tags)
                SELECT n.id, n.title,
                    {},
                    COALESCE({}, '')
                FROM note n WHERE n.id = new.note_id AND n.deleted_at IS NULL;
            END;
            "#,
            clean_content("n.content"),
            tags_for("new.note_id"),
            clean_content("n.content"),
            tags_for("new.note_id"),
        );
        db.execute_unprepared(&note_tags_insert).await?;

        // note_tags 删除触发器（修复：delete 使用 clean_content）
        let note_tags_delete = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_tags_fts_delete AFTER DELETE ON note_tags
            BEGIN
                INSERT INTO note_fts(note_fts, rowid, title, content, tags)
                SELECT 'delete', n.id, n.title,
                    {},
                    COALESCE({}, '')
                FROM note n WHERE n.id = old.note_id AND n.deleted_at IS NULL;
                INSERT OR IGNORE INTO note_fts(rowid, title, content, tags)
                SELECT n.id, n.title,
                    {},
                    COALESCE({}, '')
                FROM note n WHERE n.id = old.note_id AND n.deleted_at IS NULL;
            END;
            "#,
            clean_content("n.content"),
            tags_for("old.note_id"),
            clean_content("n.content"),
            tags_for("old.note_id"),
        );
        db.execute_unprepared(&note_tags_delete).await?;

        tracing::info!("FTS5 触发器修复完成：delete 操作统一使用 clean_content，加密前缀支持 V1+V2");

        // PostgreSQL: 补充 note_attachment 表 id 列类型修复（m20260329 遗漏）
        if backend == sea_orm::DatabaseBackend::Postgres {
            db.execute_unprepared(
                "ALTER TABLE note_attachment ALTER COLUMN id SET DATA TYPE BIGINT",
            )
            .await?;
            tracing::info!("PostgreSQL: note_attachment.id 类型修复为 BIGINT");
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        if backend != sea_orm::DatabaseBackend::Sqlite {
            return Ok(());
        }

        for trigger in [
            "note_fts_insert",
            "note_fts_update",
            "note_fts_delete",
            "note_tags_fts_insert",
            "note_tags_fts_delete",
        ] {
            db.execute_unprepared(&format!("DROP TRIGGER IF EXISTS {};", trigger))
                .await?;
        }

        db.execute_unprepared("DROP TABLE IF EXISTS note_fts;")
            .await?;

        Ok(())
    }
}
