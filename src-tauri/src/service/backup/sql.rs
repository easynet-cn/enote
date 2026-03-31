use std::io::{BufWriter, Write};

use sea_orm::*;
use tracing::info;

use super::{clear_tables, escape_sql, format_dt, BATCH_SIZE, DT_FMT};
use crate::entity::{note, note_history, note_tags, notebook, tag};

pub async fn export_sql(db: &DatabaseConnection, path: &str) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut w = BufWriter::new(file);

    writeln!(w, "-- ENote Database Backup")?;
    writeln!(w, "-- Generated: {}\n", chrono::Local::now().format(DT_FMT))?;

    // notebook（小表，直接全量）
    writeln!(w, "-- Table: notebook")?;
    for m in notebook::Entity::find().all(db).await? {
        writeln!(
            w,
            "INSERT INTO notebook (id, parent_id, name, description, icon, cls, sort_order, create_time, update_time) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {});",
            m.id,
            m.parent_id,
            escape_sql(&m.name),
            escape_sql(&m.description),
            escape_sql(&m.icon),
            escape_sql(&m.cls),
            m.sort_order,
            escape_sql(&format_dt(&m.create_time)),
            escape_sql(&format_dt(&m.update_time)),
        )?;
    }
    writeln!(w)?;

    // tag（小表，直接全量）
    writeln!(w, "-- Table: tag")?;
    for m in tag::Entity::find().all(db).await? {
        writeln!(
            w,
            "INSERT INTO tag (id, name, icon, cls, sort_order, create_time, update_time) VALUES ({}, {}, {}, {}, {}, {}, {});",
            m.id,
            escape_sql(&m.name),
            escape_sql(&m.icon),
            escape_sql(&m.cls),
            m.sort_order,
            escape_sql(&format_dt(&m.create_time)),
            escape_sql(&format_dt(&m.update_time)),
        )?;
    }
    writeln!(w)?;

    // note（大表，分页流式写入）
    writeln!(w, "-- Table: note")?;
    let mut paginator = note::Entity::find().paginate(db, BATCH_SIZE);
    while let Some(batch) = paginator.fetch_and_next().await? {
        for m in &batch {
            writeln!(
                w,
                "INSERT INTO note (id, notebook_id, title, content, content_type, create_time, update_time) VALUES ({}, {}, {}, {}, {}, {}, {});",
                m.id,
                m.notebook_id,
                escape_sql(&m.title),
                escape_sql(&m.content),
                m.content_type,
                escape_sql(&format_dt(&m.create_time)),
                escape_sql(&format_dt(&m.update_time)),
            )?;
        }
    }
    writeln!(w)?;

    // note_tags（大表，分页流式写入）
    writeln!(w, "-- Table: note_tags")?;
    let mut paginator = note_tags::Entity::find().paginate(db, BATCH_SIZE);
    while let Some(batch) = paginator.fetch_and_next().await? {
        for m in &batch {
            writeln!(
                w,
                "INSERT INTO note_tags (id, note_id, tag_id, sort_order, create_time, update_time) VALUES ({}, {}, {}, {}, {}, {});",
                m.id,
                m.note_id,
                m.tag_id,
                m.sort_order,
                escape_sql(&format_dt(&m.create_time)),
                escape_sql(&format_dt(&m.update_time)),
            )?;
        }
    }
    writeln!(w)?;

    // note_history（大表，分页流式写入）
    writeln!(w, "-- Table: note_history")?;
    let mut paginator = note_history::Entity::find().paginate(db, BATCH_SIZE);
    while let Some(batch) = paginator.fetch_and_next().await? {
        for m in &batch {
            writeln!(
                w,
                "INSERT INTO note_history (id, note_id, old_content, new_content, extra, operate_type, operate_time, create_time) VALUES ({}, {}, {}, {}, {}, {}, {}, {});",
                m.id,
                m.note_id,
                escape_sql(&m.old_content),
                escape_sql(&m.new_content),
                escape_sql(&m.extra),
                m.operate_type,
                escape_sql(&format_dt(&m.operate_time)),
                escape_sql(&format_dt(&m.create_time)),
            )?;
        }
    }

    w.flush()?;
    info!("SQL backup export completed: {}", path);
    Ok(())
}

pub async fn import_sql(db: &DatabaseConnection, path: &str) -> anyhow::Result<()> {
    let content = tokio::fs::read_to_string(path).await?;
    let statements = split_sql_statements(&content);

    if statements.is_empty() {
        return Err(crate::error::AppError::code("NO_VALID_SQL_STATEMENTS").into());
    }

    let backend = db.get_database_backend();
    let txn = db.begin().await?;
    clear_tables(&txn).await?;

    for stmt in &statements {
        txn.execute(Statement::from_string(backend, stmt.to_owned()))
            .await
            .map_err(|e| {
                anyhow::Error::from(crate::error::AppError::code_with_args(
                    "SQL_EXEC_FAILED",
                    vec![e.to_string()],
                ))
            })?;
    }

    txn.commit().await?;
    info!("SQL backup import completed: {}", path);
    Ok(())
}

fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut chars = sql.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_string {
            current.push(ch);
            if ch == '\'' {
                if chars.peek() == Some(&'\'') {
                    if let Some(ch) = chars.next() {
                        current.push(ch);
                    }
                } else {
                    in_string = false;
                }
            }
        } else {
            match ch {
                '\'' => {
                    in_string = true;
                    current.push(ch);
                }
                ';' => {
                    let stmt = current.trim().to_string();
                    if !stmt.is_empty()
                        && stmt.len() > 6
                        && stmt[..6].eq_ignore_ascii_case("INSERT")
                    {
                        statements.push(stmt);
                    }
                    current.clear();
                }
                '-' if chars.peek() == Some(&'-') => {
                    chars.next();
                    for ch in chars.by_ref() {
                        if ch == '\n' {
                            break;
                        }
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }
    }

    let stmt = current.trim().to_string();
    if !stmt.is_empty() && stmt.len() > 6 && stmt[..6].eq_ignore_ascii_case("INSERT") {
        statements.push(stmt);
    }

    statements
}
