//! 升级 FTS5 全文搜索：增加 tags 列
//!
//! - FTS5 表新增 `tags` 列，索引标签名称
//! - 更新触发器：note 变更和 note_tags 变更时同步更新 FTS
//! - 加密内容（以 ENOTE_ENC_V1: 开头）不索引 content，存空字符串

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 构建获取笔记关联标签名称的子查询 SQL
const TAGS_SUBQUERY: &str =
    "(SELECT group_concat(t.name, ' ') FROM note_tags nt JOIN tag t ON t.id = nt.tag_id WHERE nt.note_id = {note_id})";

/// 构建去除 HTML 标签的 content 表达式（跳过加密内容）
const CLEAN_CONTENT_EXPR: &str = r#"
    CASE WHEN {content} LIKE 'ENOTE_ENC_V1:%' THEN ''
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
            tracing::info!("跳过 FTS 升级：当前数据库不是 SQLite");
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

        // 删除旧的 FTS 表
        db.execute_unprepared("DROP TABLE IF EXISTS note_fts;")
            .await?;

        // 创建新的 FTS5 表，增加 tags 列
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

        // 同步现有数据到 FTS 表
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

        // note 插入触发器
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

        // note 更新触发器
        let update_trigger = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_update AFTER UPDATE ON note BEGIN
                INSERT INTO note_fts(note_fts, rowid, title, content, tags)
                VALUES ('delete', old.id, old.title, old.content,
                    COALESCE({}, ''));
                INSERT OR IGNORE INTO note_fts(rowid, title, content, tags)
                SELECT new.id, new.title,
                    {},
                    COALESCE({}, '')
                WHERE new.deleted_at IS NULL;
            END;
            "#,
            tags_for("old.id"),
            clean_content("new.content"),
            tags_for("new.id"),
        );
        db.execute_unprepared(&update_trigger).await?;

        // note 删除触发器
        let delete_trigger = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_delete AFTER DELETE ON note BEGIN
                INSERT INTO note_fts(note_fts, rowid, title, content, tags)
                VALUES ('delete', old.id, old.title, old.content,
                    COALESCE({}, ''));
            END;
            "#,
            tags_for("old.id"),
        );
        db.execute_unprepared(&delete_trigger).await?;

        // note_tags 插入触发器：标签关联变更时更新 FTS
        let note_tags_insert = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_tags_fts_insert AFTER INSERT ON note_tags
            BEGIN
                -- 先删除旧的 FTS 记录
                INSERT INTO note_fts(note_fts, rowid, title, content, tags)
                SELECT 'delete', n.id, n.title, n.content,
                    COALESCE({}, '')
                FROM note n WHERE n.id = new.note_id AND n.deleted_at IS NULL;
                -- 重新插入含新标签的记录
                INSERT OR IGNORE INTO note_fts(rowid, title, content, tags)
                SELECT n.id, n.title,
                    {},
                    COALESCE({}, '')
                FROM note n WHERE n.id = new.note_id AND n.deleted_at IS NULL;
            END;
            "#,
            // 删除时用旧标签（但 note_tags 还没有 new 行，所以这里有个时序问题）
            // 实际上 AFTER INSERT 时 new 行已在表中，所以 tags_for 会包含新标签
            // 但删除旧记录时需要用之前的内容，这里用 note 表的 content
            tags_for("new.note_id"),
            clean_content("n.content"),
            tags_for("new.note_id"),
        );
        db.execute_unprepared(&note_tags_insert).await?;

        // note_tags 删除触发器
        let note_tags_delete = format!(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_tags_fts_delete AFTER DELETE ON note_tags
            BEGIN
                -- 先删除旧的 FTS 记录
                INSERT INTO note_fts(note_fts, rowid, title, content, tags)
                SELECT 'delete', n.id, n.title, n.content,
                    COALESCE({}, '')
                FROM note n WHERE n.id = old.note_id AND n.deleted_at IS NULL;
                -- 重新插入含更新后标签的记录
                INSERT OR IGNORE INTO note_fts(rowid, title, content, tags)
                SELECT n.id, n.title,
                    {},
                    COALESCE({}, '')
                FROM note n WHERE n.id = old.note_id AND n.deleted_at IS NULL;
            END;
            "#,
            tags_for("old.note_id"),
            clean_content("n.content"),
            tags_for("old.note_id"),
        );
        db.execute_unprepared(&note_tags_delete).await?;

        tracing::info!("FTS5 升级完成：新增 tags 列，加密内容跳过索引");

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
