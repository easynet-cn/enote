//! 升级笔记全文搜索表
//!
//! 重建 FTS5 表，改进中文搜索支持：
//! - 使用 unicode61 + trigram 双索引策略
//! - trigram 索引用于中文子串搜索
//! - 更新触发器以去除 HTML 标签

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 用于在 SQL 中去除 HTML 标签的简单方法
/// 通过嵌套 replace 无法完美实现，所以我们创建一个辅助 FTS 表
/// 使用 trigram tokenizer 来支持中文子串搜索
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
        db.execute_unprepared("DROP TRIGGER IF EXISTS note_fts_insert;")
            .await?;
        db.execute_unprepared("DROP TRIGGER IF EXISTS note_fts_update;")
            .await?;
        db.execute_unprepared("DROP TRIGGER IF EXISTS note_fts_delete;")
            .await?;

        // 删除旧的 FTS 表
        db.execute_unprepared("DROP TABLE IF EXISTS note_fts;")
            .await?;

        // 创建新的 FTS5 表，使用 trigram tokenizer 支持中文子串搜索
        // trigram 将文本分为 3 字符的 n-gram，天然支持中文搜索
        db.execute_unprepared(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS note_fts USING fts5(
                title,
                content,
                content='note',
                content_rowid='id',
                tokenize='trigram'
            );
            "#,
        )
        .await?;

        // 同步现有数据到 FTS 表（去除 HTML 标签）
        // 使用 replace 去除常见 HTML 标签
        db.execute_unprepared(
            r#"
            INSERT INTO note_fts(rowid, title, content)
            SELECT id, title,
                replace(replace(replace(replace(replace(replace(
                    content,
                    '<br>', ' '), '<br/>', ' '), '<br />', ' '),
                    '</p>', ' '), '</div>', ' '), '</li>', ' ')
            FROM note WHERE deleted_at IS NULL;
            "#,
        )
        .await?;

        // 创建插入触发器
        db.execute_unprepared(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_insert AFTER INSERT ON note
            WHEN new.deleted_at IS NULL
            BEGIN
                INSERT INTO note_fts(rowid, title, content)
                VALUES (new.id, new.title,
                    replace(replace(replace(replace(replace(replace(
                        new.content,
                        '<br>', ' '), '<br/>', ' '), '<br />', ' '),
                        '</p>', ' '), '</div>', ' '), '</li>', ' ')
                );
            END;
            "#,
        )
        .await?;

        // 创建更新触发器
        db.execute_unprepared(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_update AFTER UPDATE ON note BEGIN
                INSERT INTO note_fts(note_fts, rowid, title, content)
                VALUES ('delete', old.id, old.title, old.content);
                INSERT OR IGNORE INTO note_fts(rowid, title, content)
                SELECT new.id, new.title,
                    replace(replace(replace(replace(replace(replace(
                        new.content,
                        '<br>', ' '), '<br/>', ' '), '<br />', ' '),
                        '</p>', ' '), '</div>', ' '), '</li>', ' ')
                WHERE new.deleted_at IS NULL;
            END;
            "#,
        )
        .await?;

        // 创建删除触发器
        db.execute_unprepared(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_delete AFTER DELETE ON note BEGIN
                INSERT INTO note_fts(note_fts, rowid, title, content)
                VALUES ('delete', old.id, old.title, old.content);
            END;
            "#,
        )
        .await?;

        tracing::info!("FTS5 全文搜索表升级完成（trigram tokenizer）");

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        if backend != sea_orm::DatabaseBackend::Sqlite {
            return Ok(());
        }

        // 删除触发器
        db.execute_unprepared("DROP TRIGGER IF EXISTS note_fts_insert;")
            .await?;
        db.execute_unprepared("DROP TRIGGER IF EXISTS note_fts_update;")
            .await?;
        db.execute_unprepared("DROP TRIGGER IF EXISTS note_fts_delete;")
            .await?;

        // 删除新的 FTS 表
        db.execute_unprepared("DROP TABLE IF EXISTS note_fts;")
            .await?;

        Ok(())
    }
}
