use std::io::Read;

use anyhow::Context;
use chrono::NaiveDateTime;
use sea_orm::*;
use tracing::info;

use super::{BATCH_SIZE, BackupData, clear_tables, format_dt, parse_dt, restore_data};
use crate::entity::{note, note_history, note_tags, notebook, tag, todo, todo_list};

/// 安全解析 CSV 字段为 i64，解析失败时返回带上下文的错误
fn parse_i64(val: &str, table: &str, field: &str) -> anyhow::Result<i64> {
    val.parse::<i64>().with_context(|| {
        format!(
            "CSV import: invalid integer in {}.{}: '{}'",
            table, field, val
        )
    })
}

/// 安全解析 CSV 字段为 i32，解析失败时返回带上下文的错误
fn parse_i32(val: &str, table: &str, field: &str) -> anyhow::Result<i32> {
    val.parse::<i32>().with_context(|| {
        format!(
            "CSV import: invalid integer in {}.{}: '{}'",
            table, field, val
        )
    })
}

/// 解析可选 i64：空串视为 None
fn parse_opt_i64(val: &str, table: &str, field: &str) -> anyhow::Result<Option<i64>> {
    if val.trim().is_empty() {
        return Ok(None);
    }
    parse_i64(val, table, field).map(Some)
}

/// 解析可选时间：空串视为 None
fn parse_opt_dt(val: &str) -> anyhow::Result<Option<NaiveDateTime>> {
    if val.trim().is_empty() {
        return Ok(None);
    }
    parse_dt(val).map(Some)
}

/// 可选时间转 CSV 字符串（None 输出空串）
fn opt_dt_str(dt: Option<NaiveDateTime>) -> String {
    dt.map(|d| format_dt(&d)).unwrap_or_default()
}

/// 可选整数转 CSV 字符串（None 输出空串）
fn opt_i64_str(v: Option<i64>) -> String {
    v.map(|n| n.to_string()).unwrap_or_default()
}

pub async fn export_csv(db: &DatabaseConnection, path: &str) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // notebook.csv（小表）
    {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.write_record([
            "id",
            "parent_id",
            "name",
            "description",
            "icon",
            "cls",
            "sort_order",
            "create_time",
            "update_time",
        ])?;
        for m in notebook::Entity::find().all(db).await? {
            wtr.write_record(&[
                m.id.to_string(),
                m.parent_id.to_string(),
                m.name,
                m.description,
                m.icon,
                m.cls,
                m.sort_order.to_string(),
                format_dt(&m.create_time),
                format_dt(&m.update_time),
            ])?;
        }
        zip.start_file("notebook.csv", opts)?;
        std::io::Write::write_all(&mut zip, &wtr.into_inner()?)?;
    }

    // tag.csv（小表）
    {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.write_record([
            "id",
            "name",
            "icon",
            "cls",
            "sort_order",
            "create_time",
            "update_time",
        ])?;
        for m in tag::Entity::find().all(db).await? {
            wtr.write_record(&[
                m.id.to_string(),
                m.name,
                m.icon,
                m.cls,
                m.sort_order.to_string(),
                format_dt(&m.create_time),
                format_dt(&m.update_time),
            ])?;
        }
        zip.start_file("tag.csv", opts)?;
        std::io::Write::write_all(&mut zip, &wtr.into_inner()?)?;
    }

    // note.csv（大表，分页读取写入 CSV 缓冲）
    {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.write_record([
            "id",
            "notebook_id",
            "title",
            "content",
            "content_type",
            "create_time",
            "update_time",
        ])?;
        let mut paginator = note::Entity::find().paginate(db, BATCH_SIZE);
        while let Some(batch) = paginator.fetch_and_next().await? {
            for m in batch {
                wtr.write_record(&[
                    m.id.to_string(),
                    m.notebook_id.to_string(),
                    m.title,
                    m.content,
                    m.content_type.to_string(),
                    format_dt(&m.create_time),
                    format_dt(&m.update_time),
                ])?;
            }
        }
        zip.start_file("note.csv", opts)?;
        std::io::Write::write_all(&mut zip, &wtr.into_inner()?)?;
    }

    // note_tags.csv（大表，分页读取）
    {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.write_record([
            "id",
            "note_id",
            "tag_id",
            "sort_order",
            "create_time",
            "update_time",
        ])?;
        let mut paginator = note_tags::Entity::find().paginate(db, BATCH_SIZE);
        while let Some(batch) = paginator.fetch_and_next().await? {
            for m in batch {
                wtr.write_record(&[
                    m.id.to_string(),
                    m.note_id.to_string(),
                    m.tag_id.to_string(),
                    m.sort_order.to_string(),
                    format_dt(&m.create_time),
                    format_dt(&m.update_time),
                ])?;
            }
        }
        zip.start_file("note_tags.csv", opts)?;
        std::io::Write::write_all(&mut zip, &wtr.into_inner()?)?;
    }

    // note_history.csv（大表，分页读取）
    {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        wtr.write_record([
            "id",
            "note_id",
            "old_content",
            "new_content",
            "extra",
            "operate_type",
            "operate_time",
            "create_time",
        ])?;
        let mut paginator = note_history::Entity::find().paginate(db, BATCH_SIZE);
        while let Some(batch) = paginator.fetch_and_next().await? {
            for m in batch {
                wtr.write_record(&[
                    m.id.to_string(),
                    m.note_id.to_string(),
                    m.old_content,
                    m.new_content,
                    m.extra,
                    m.operate_type.to_string(),
                    format_dt(&m.operate_time),
                    format_dt(&m.create_time),
                ])?;
            }
        }
        zip.start_file("note_history.csv", opts)?;
        std::io::Write::write_all(&mut zip, &wtr.into_inner()?)?;
    }

    // todo_list
    {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "id",
            "name",
            "icon",
            "color",
            "sort_order",
            "mcp_access",
            "create_time",
            "update_time",
        ])?;
        for m in todo_list::Entity::find().all(db).await? {
            wtr.write_record(&[
                m.id.to_string(),
                m.name,
                m.icon,
                m.color,
                m.sort_order.to_string(),
                m.mcp_access.to_string(),
                format_dt(&m.create_time),
                format_dt(&m.update_time),
            ])?;
        }
        zip.start_file("todo_list.csv", opts)?;
        std::io::Write::write_all(&mut zip, &wtr.into_inner()?)?;
    }

    // todo
    {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "id",
            "title",
            "description",
            "is_completed",
            "priority",
            "due_date",
            "completed_at",
            "list_id",
            "sort_order",
            "create_time",
            "update_time",
            "deleted_at",
            "start_date",
            "remind_at",
            "is_reminded",
            "parent_id",
            "note_id",
            "recurrence_type",
            "recurrence_interval",
            "recurrence_end_date",
            "mcp_access",
        ])?;
        let mut paginator = todo::Entity::find().paginate(db, BATCH_SIZE);
        while let Some(batch) = paginator.fetch_and_next().await? {
            for m in batch {
                wtr.write_record(&[
                    m.id.to_string(),
                    m.title,
                    m.description,
                    m.is_completed.to_string(),
                    m.priority.to_string(),
                    opt_dt_str(m.due_date),
                    opt_dt_str(m.completed_at),
                    opt_i64_str(m.list_id),
                    m.sort_order.to_string(),
                    format_dt(&m.create_time),
                    format_dt(&m.update_time),
                    opt_dt_str(m.deleted_at),
                    opt_dt_str(m.start_date),
                    opt_dt_str(m.remind_at),
                    m.is_reminded.to_string(),
                    m.parent_id.to_string(),
                    opt_i64_str(m.note_id),
                    m.recurrence_type.to_string(),
                    m.recurrence_interval.to_string(),
                    opt_dt_str(m.recurrence_end_date),
                    m.mcp_access.to_string(),
                ])?;
            }
        }
        zip.start_file("todo.csv", opts)?;
        std::io::Write::write_all(&mut zip, &wtr.into_inner()?)?;
    }

    zip.finish()?;
    info!("CSV backup export completed: {}", path);
    Ok(())
}

pub async fn import_csv(db: &DatabaseConnection, path: &str) -> anyhow::Result<()> {
    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut data = BackupData {
        notebooks: Vec::new(),
        tags: Vec::new(),
        notes: Vec::new(),
        note_tags: Vec::new(),
        note_histories: Vec::new(),
        todo_lists: Vec::new(),
        todos: Vec::new(),
    };

    // notebook
    if let Ok(mut f) = archive.by_name("notebook.csv") {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.records() {
            let r = result?;
            if r.len() < 9 {
                continue;
            }
            data.notebooks.push(notebook::Model {
                id: parse_i64(&r[0], "notebook", "id")?,
                parent_id: parse_i64(&r[1], "notebook", "parent_id")?,
                name: r[2].to_string(),
                description: r[3].to_string(),
                icon: r[4].to_string(),
                cls: r[5].to_string(),
                sort_order: parse_i32(&r[6], "notebook", "sort_order")?,
                mcp_access: 0,
                create_time: parse_dt(&r[7])?,
                update_time: parse_dt(&r[8])?,
            });
        }
    }

    // tag
    if let Ok(mut f) = archive.by_name("tag.csv") {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.records() {
            let r = result?;
            if r.len() < 7 {
                continue;
            }
            data.tags.push(tag::Model {
                id: parse_i64(&r[0], "tag", "id")?,
                name: r[1].to_string(),
                icon: r[2].to_string(),
                cls: r[3].to_string(),
                sort_order: parse_i32(&r[4], "tag", "sort_order")?,
                mcp_access: 0,
                create_time: parse_dt(&r[5])?,
                update_time: parse_dt(&r[6])?,
            });
        }
    }

    // note
    if let Ok(mut f) = archive.by_name("note.csv") {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.records() {
            let r = result?;
            if r.len() < 7 {
                continue;
            }
            data.notes.push(note::Model {
                id: parse_i64(&r[0], "note", "id")?,
                notebook_id: parse_i64(&r[1], "note", "notebook_id")?,
                title: r[2].to_string(),
                content: r[3].to_string(),
                content_type: parse_i32(&r[4], "note", "content_type")?,
                is_pinned: if r.len() > 7 {
                    r[7].parse().unwrap_or(0)
                } else {
                    0
                },
                is_starred: if r.len() > 8 {
                    r[8].parse().unwrap_or(0)
                } else {
                    0
                },
                mcp_access: 0,
                create_time: parse_dt(&r[5])?,
                update_time: parse_dt(&r[6])?,
                deleted_at: None,
            });
        }
    }

    // note_tags
    if let Ok(mut f) = archive.by_name("note_tags.csv") {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.records() {
            let r = result?;
            if r.len() < 6 {
                continue;
            }
            data.note_tags.push(note_tags::Model {
                id: parse_i64(&r[0], "note_tags", "id")?,
                note_id: parse_i64(&r[1], "note_tags", "note_id")?,
                tag_id: parse_i64(&r[2], "note_tags", "tag_id")?,
                sort_order: parse_i32(&r[3], "note_tags", "sort_order")?,
                create_time: parse_dt(&r[4])?,
                update_time: parse_dt(&r[5])?,
            });
        }
    }

    // note_history
    if let Ok(mut f) = archive.by_name("note_history.csv") {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.records() {
            let r = result?;
            if r.len() < 8 {
                continue;
            }
            data.note_histories.push(note_history::Model {
                id: parse_i64(&r[0], "note_history", "id")?,
                note_id: parse_i64(&r[1], "note_history", "note_id")?,
                old_content: r[2].to_string(),
                new_content: r[3].to_string(),
                extra: r[4].to_string(),
                operate_type: parse_i32(&r[5], "note_history", "operate_type")?,
                operate_source: 0,
                operate_time: parse_dt(&r[6])?,
                create_time: parse_dt(&r[7])?,
            });
        }
    }

    // todo_list
    if let Ok(mut f) = archive.by_name("todo_list.csv") {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.records() {
            let r = result?;
            if r.len() < 8 {
                continue;
            }
            data.todo_lists.push(todo_list::Model {
                id: parse_i64(&r[0], "todo_list", "id")?,
                name: r[1].to_string(),
                icon: r[2].to_string(),
                color: r[3].to_string(),
                sort_order: parse_i32(&r[4], "todo_list", "sort_order")?,
                mcp_access: parse_i32(&r[5], "todo_list", "mcp_access")?,
                create_time: parse_dt(&r[6])?,
                update_time: parse_dt(&r[7])?,
            });
        }
    }

    // todo
    if let Ok(mut f) = archive.by_name("todo.csv") {
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.records() {
            let r = result?;
            if r.len() < 21 {
                continue;
            }
            data.todos.push(todo::Model {
                id: parse_i64(&r[0], "todo", "id")?,
                title: r[1].to_string(),
                description: r[2].to_string(),
                is_completed: parse_i32(&r[3], "todo", "is_completed")?,
                priority: parse_i32(&r[4], "todo", "priority")?,
                due_date: parse_opt_dt(&r[5])?,
                completed_at: parse_opt_dt(&r[6])?,
                list_id: parse_opt_i64(&r[7], "todo", "list_id")?,
                sort_order: parse_i32(&r[8], "todo", "sort_order")?,
                create_time: parse_dt(&r[9])?,
                update_time: parse_dt(&r[10])?,
                deleted_at: parse_opt_dt(&r[11])?,
                start_date: parse_opt_dt(&r[12])?,
                remind_at: parse_opt_dt(&r[13])?,
                is_reminded: parse_i32(&r[14], "todo", "is_reminded")?,
                parent_id: parse_i64(&r[15], "todo", "parent_id")?,
                note_id: parse_opt_i64(&r[16], "todo", "note_id")?,
                recurrence_type: parse_i32(&r[17], "todo", "recurrence_type")?,
                recurrence_interval: parse_i32(&r[18], "todo", "recurrence_interval")?,
                recurrence_end_date: parse_opt_dt(&r[19])?,
                mcp_access: parse_i32(&r[20], "todo", "mcp_access")?,
            });
        }
    }

    let txn = db.begin().await?;
    clear_tables(&txn).await?;
    restore_data(&txn, &data).await?;
    txn.commit().await?;
    info!("CSV backup import completed: {}", path);
    Ok(())
}
