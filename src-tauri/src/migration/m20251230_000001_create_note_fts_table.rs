//! 创建笔记全文搜索表迁移
//!
//! 本迁移为 SQLite 数据库创建 FTS5 全文搜索虚拟表，
//! 并设置触发器以保持与 note 表的数据同步。
//!
//! # FTS5 优势
//! - 比 LIKE 查询快 10-100 倍
//! - 支持中文分词（使用 unicode61 分词器）
//! - 支持高级搜索语法（AND、OR、NOT 等）

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 检查是否为 SQLite 数据库
        let backend = db.get_database_backend();
        if backend != sea_orm::DatabaseBackend::Sqlite {
            // 非 SQLite 数据库跳过 FTS5 创建
            tracing::info!("跳过 FTS5 创建：当前数据库不是 SQLite");
            return Ok(());
        }

        // 创建 FTS5 虚拟表
        // 使用 unicode61 分词器支持中文
        db.execute_unprepared(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS note_fts USING fts5(
                title,
                content,
                content='note',
                content_rowid='id',
                tokenize='unicode61'
            );
            "#,
        )
        .await?;

        // 同步现有数据到 FTS 表
        db.execute_unprepared(
            r#"
            INSERT INTO note_fts(rowid, title, content)
            SELECT id, title, content FROM note;
            "#,
        )
        .await?;

        // 创建插入触发器
        db.execute_unprepared(
            r#"
            CREATE TRIGGER IF NOT EXISTS note_fts_insert AFTER INSERT ON note BEGIN
                INSERT INTO note_fts(rowid, title, content)
                VALUES (new.id, new.title, new.content);
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
                INSERT INTO note_fts(rowid, title, content)
                VALUES (new.id, new.title, new.content);
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

        tracing::info!("FTS5 全文搜索表创建完成");

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

        // 删除 FTS5 虚拟表
        db.execute_unprepared("DROP TABLE IF EXISTS note_fts;")
            .await?;

        Ok(())
    }
}
