//! 数据库备份与恢复服务
//!
//! 支持 SQL、Excel、CSV 三种格式的导出和导入

use std::io::{BufWriter, Read, Write};
use std::path::Path;

use chrono::NaiveDateTime;
use sea_orm::*;
use tracing::info;

use crate::entity::{note, note_history, note_tags, notebook, tag};

/// 分页批次大小
const BATCH_SIZE: u64 = 500;

const DT_FMT: &str = "%Y-%m-%d %H:%M:%S";

// ============================================================================
// 数据结构
// ============================================================================

struct BackupData {
    notebooks: Vec<notebook::Model>,
    tags: Vec<tag::Model>,
    notes: Vec<note::Model>,
    note_tags: Vec<note_tags::Model>,
    note_histories: Vec<note_history::Model>,
}

// ============================================================================
// 通用工具函数
// ============================================================================

fn format_dt(dt: &NaiveDateTime) -> String {
    dt.format(DT_FMT).to_string()
}

fn parse_dt(s: &str) -> anyhow::Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s.trim(), DT_FMT)
        .map_err(|e| anyhow::anyhow!("日期解析失败 '{}': {}", s, e))
}

fn escape_sql(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

async fn clear_tables(txn: &impl ConnectionTrait) -> anyhow::Result<()> {
    note_tags::Entity::delete_many().exec(txn).await?;
    note_history::Entity::delete_many().exec(txn).await?;
    note::Entity::delete_many().exec(txn).await?;
    tag::Entity::delete_many().exec(txn).await?;
    notebook::Entity::delete_many().exec(txn).await?;
    Ok(())
}

async fn restore_data(txn: &impl ConnectionTrait, data: &BackupData) -> anyhow::Result<()> {
    use sea_orm::ActiveValue::Set;

    // 批量插入 notebooks
    if !data.notebooks.is_empty() {
        let models: Vec<notebook::ActiveModel> = data
            .notebooks
            .iter()
            .map(|m| notebook::ActiveModel {
                id: Set(m.id),
                parent_id: Set(m.parent_id),
                name: Set(m.name.clone()),
                description: Set(m.description.clone()),
                icon: Set(m.icon.clone()),
                cls: Set(m.cls.clone()),
                sort_order: Set(m.sort_order),
                mcp_access: Set(m.mcp_access),
                create_time: Set(m.create_time),
                update_time: Set(m.update_time),
            })
            .collect();
        notebook::Entity::insert_many(models).exec(txn).await?;
    }

    // 批量插入 tags
    if !data.tags.is_empty() {
        let models: Vec<tag::ActiveModel> = data
            .tags
            .iter()
            .map(|m| tag::ActiveModel {
                id: Set(m.id),
                name: Set(m.name.clone()),
                icon: Set(m.icon.clone()),
                cls: Set(m.cls.clone()),
                sort_order: Set(m.sort_order),
                mcp_access: Set(m.mcp_access),
                create_time: Set(m.create_time),
                update_time: Set(m.update_time),
            })
            .collect();
        tag::Entity::insert_many(models).exec(txn).await?;
    }

    // 批量插入 notes（分批 500 条，避免大数据集内存压力）
    for chunk in data.notes.chunks(500) {
        let models: Vec<note::ActiveModel> = chunk
            .iter()
            .map(|m| note::ActiveModel {
                id: Set(m.id),
                notebook_id: Set(m.notebook_id),
                title: Set(m.title.clone()),
                content: Set(m.content.clone()),
                content_type: Set(m.content_type),
                is_pinned: Set(m.is_pinned),
                mcp_access: Set(m.mcp_access),
                create_time: Set(m.create_time),
                update_time: Set(m.update_time),
                deleted_at: Set(m.deleted_at),
            })
            .collect();
        note::Entity::insert_many(models).exec(txn).await?;
    }

    // 批量插入 note_tags
    if !data.note_tags.is_empty() {
        for chunk in data.note_tags.chunks(500) {
            let models: Vec<note_tags::ActiveModel> = chunk
                .iter()
                .map(|m| note_tags::ActiveModel {
                    id: Set(m.id),
                    note_id: Set(m.note_id),
                    tag_id: Set(m.tag_id),
                    sort_order: Set(m.sort_order),
                    create_time: Set(m.create_time),
                    update_time: Set(m.update_time),
                })
                .collect();
            note_tags::Entity::insert_many(models).exec(txn).await?;
        }
    }

    // 批量插入 note_histories
    for chunk in data.note_histories.chunks(500) {
        let models: Vec<note_history::ActiveModel> = chunk
            .iter()
            .map(|m| note_history::ActiveModel {
                id: Set(m.id),
                note_id: Set(m.note_id),
                old_content: Set(m.old_content.clone()),
                new_content: Set(m.new_content.clone()),
                extra: Set(m.extra.clone()),
                operate_type: Set(m.operate_type),
                operate_source: Set(m.operate_source),
                operate_time: Set(m.operate_time),
                create_time: Set(m.create_time),
            })
            .collect();
        note_history::Entity::insert_many(models).exec(txn).await?;
    }

    Ok(())
}

// ============================================================================
// SQL 导出/导入
// ============================================================================

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
    info!("SQL 备份导出完成: {}", path);
    Ok(())
}

pub async fn import_sql(db: &DatabaseConnection, path: &str) -> anyhow::Result<()> {
    let content = tokio::fs::read_to_string(path).await?;
    let statements = split_sql_statements(&content);

    if statements.is_empty() {
        anyhow::bail!("SQL 文件中没有找到有效的 INSERT 语句");
    }

    let backend = db.get_database_backend();
    let txn = db.begin().await?;
    clear_tables(&txn).await?;

    for stmt in &statements {
        txn.execute(Statement::from_string(backend, stmt.to_owned()))
            .await
            .map_err(|e| anyhow::anyhow!("执行 SQL 失败: {}", e))?;
    }

    txn.commit().await?;
    info!("SQL 备份导入完成: {}", path);
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

// ============================================================================
// Excel 导出/导入
// ============================================================================

pub async fn export_excel(db: &DatabaseConnection, path: &str) -> anyhow::Result<()> {
    let mut workbook = rust_xlsxwriter::Workbook::new();

    // notebook（小表）
    {
        let sheet = workbook.add_worksheet();
        sheet.set_name("notebook")?;
        for (i, h) in [
            "id",
            "parent_id",
            "name",
            "description",
            "icon",
            "cls",
            "sort_order",
            "create_time",
            "update_time",
        ]
        .iter()
        .enumerate()
        {
            sheet.write_string(0, i as u16, *h)?;
        }
        for (r, m) in notebook::Entity::find().all(db).await?.iter().enumerate() {
            let row = (r + 1) as u32;
            sheet.write_number(row, 0, m.id as f64)?;
            sheet.write_number(row, 1, m.parent_id as f64)?;
            sheet.write_string(row, 2, &m.name)?;
            sheet.write_string(row, 3, &m.description)?;
            sheet.write_string(row, 4, &m.icon)?;
            sheet.write_string(row, 5, &m.cls)?;
            sheet.write_number(row, 6, m.sort_order as f64)?;
            sheet.write_string(row, 7, format_dt(&m.create_time))?;
            sheet.write_string(row, 8, format_dt(&m.update_time))?;
        }
    }

    // tag（小表）
    {
        let sheet = workbook.add_worksheet();
        sheet.set_name("tag")?;
        for (i, h) in [
            "id",
            "name",
            "icon",
            "cls",
            "sort_order",
            "create_time",
            "update_time",
        ]
        .iter()
        .enumerate()
        {
            sheet.write_string(0, i as u16, *h)?;
        }
        for (r, m) in tag::Entity::find().all(db).await?.iter().enumerate() {
            let row = (r + 1) as u32;
            sheet.write_number(row, 0, m.id as f64)?;
            sheet.write_string(row, 1, &m.name)?;
            sheet.write_string(row, 2, &m.icon)?;
            sheet.write_string(row, 3, &m.cls)?;
            sheet.write_number(row, 4, m.sort_order as f64)?;
            sheet.write_string(row, 5, format_dt(&m.create_time))?;
            sheet.write_string(row, 6, format_dt(&m.update_time))?;
        }
    }

    // note（大表，分页读取）
    {
        let sheet = workbook.add_worksheet();
        sheet.set_name("note")?;
        for (i, h) in [
            "id",
            "notebook_id",
            "title",
            "content",
            "content_type",
            "create_time",
            "update_time",
        ]
        .iter()
        .enumerate()
        {
            sheet.write_string(0, i as u16, *h)?;
        }
        let mut row_offset: u32 = 1;
        let mut paginator = note::Entity::find().paginate(db, BATCH_SIZE);
        while let Some(batch) = paginator.fetch_and_next().await? {
            for (r, m) in batch.iter().enumerate() {
                let row = row_offset + r as u32;
                sheet.write_number(row, 0, m.id as f64)?;
                sheet.write_number(row, 1, m.notebook_id as f64)?;
                sheet.write_string(row, 2, &m.title)?;
                sheet.write_string(row, 3, &m.content)?;
                sheet.write_number(row, 4, m.content_type as f64)?;
                sheet.write_string(row, 5, format_dt(&m.create_time))?;
                sheet.write_string(row, 6, format_dt(&m.update_time))?;
            }
            row_offset += batch.len() as u32;
        }
    }

    // note_tags（大表，分页读取）
    {
        let sheet = workbook.add_worksheet();
        sheet.set_name("note_tags")?;
        for (i, h) in [
            "id",
            "note_id",
            "tag_id",
            "sort_order",
            "create_time",
            "update_time",
        ]
        .iter()
        .enumerate()
        {
            sheet.write_string(0, i as u16, *h)?;
        }
        let mut row_offset: u32 = 1;
        let mut paginator = note_tags::Entity::find().paginate(db, BATCH_SIZE);
        while let Some(batch) = paginator.fetch_and_next().await? {
            for (r, m) in batch.iter().enumerate() {
                let row = row_offset + r as u32;
                sheet.write_number(row, 0, m.id as f64)?;
                sheet.write_number(row, 1, m.note_id as f64)?;
                sheet.write_number(row, 2, m.tag_id as f64)?;
                sheet.write_number(row, 3, m.sort_order as f64)?;
                sheet.write_string(row, 4, format_dt(&m.create_time))?;
                sheet.write_string(row, 5, format_dt(&m.update_time))?;
            }
            row_offset += batch.len() as u32;
        }
    }

    // note_history（大表，分页读取）
    {
        let sheet = workbook.add_worksheet();
        sheet.set_name("note_history")?;
        for (i, h) in [
            "id",
            "note_id",
            "old_content",
            "new_content",
            "extra",
            "operate_type",
            "operate_time",
            "create_time",
        ]
        .iter()
        .enumerate()
        {
            sheet.write_string(0, i as u16, *h)?;
        }
        let mut row_offset: u32 = 1;
        let mut paginator = note_history::Entity::find().paginate(db, BATCH_SIZE);
        while let Some(batch) = paginator.fetch_and_next().await? {
            for (r, m) in batch.iter().enumerate() {
                let row = row_offset + r as u32;
                sheet.write_number(row, 0, m.id as f64)?;
                sheet.write_number(row, 1, m.note_id as f64)?;
                sheet.write_string(row, 2, &m.old_content)?;
                sheet.write_string(row, 3, &m.new_content)?;
                sheet.write_string(row, 4, &m.extra)?;
                sheet.write_number(row, 5, m.operate_type as f64)?;
                sheet.write_string(row, 6, format_dt(&m.operate_time))?;
                sheet.write_string(row, 7, format_dt(&m.create_time))?;
            }
            row_offset += batch.len() as u32;
        }
    }

    workbook.save(path)?;
    info!("Excel 备份导出完成: {}", path);
    Ok(())
}

pub async fn import_excel(db: &DatabaseConnection, path: &str) -> anyhow::Result<()> {
    use calamine::{Reader, Xlsx, open_workbook};

    let mut wb: Xlsx<_> = open_workbook(path)?;
    let mut data = BackupData {
        notebooks: Vec::new(),
        tags: Vec::new(),
        notes: Vec::new(),
        note_tags: Vec::new(),
        note_histories: Vec::new(),
    };

    if let Ok(range) = wb.worksheet_range("notebook") {
        for row in range.rows().skip(1) {
            if row.len() < 9 {
                continue;
            }
            data.notebooks.push(notebook::Model {
                id: cell_i64(&row[0]),
                parent_id: cell_i64(&row[1]),
                name: cell_str(&row[2]),
                description: cell_str(&row[3]),
                icon: cell_str(&row[4]),
                cls: cell_str(&row[5]),
                sort_order: cell_i64(&row[6]) as i32,
                mcp_access: 0,
                create_time: cell_dt(&row[7])?,
                update_time: cell_dt(&row[8])?,
            });
        }
    }

    if let Ok(range) = wb.worksheet_range("tag") {
        for row in range.rows().skip(1) {
            if row.len() < 7 {
                continue;
            }
            data.tags.push(tag::Model {
                id: cell_i64(&row[0]),
                name: cell_str(&row[1]),
                icon: cell_str(&row[2]),
                cls: cell_str(&row[3]),
                sort_order: cell_i64(&row[4]) as i32,
                mcp_access: 0,
                create_time: cell_dt(&row[5])?,
                update_time: cell_dt(&row[6])?,
            });
        }
    }

    if let Ok(range) = wb.worksheet_range("note") {
        for row in range.rows().skip(1) {
            if row.len() < 7 {
                continue;
            }
            data.notes.push(note::Model {
                id: cell_i64(&row[0]),
                notebook_id: cell_i64(&row[1]),
                title: cell_str(&row[2]),
                content: cell_str(&row[3]),
                content_type: cell_i64(&row[4]) as i32,
                is_pinned: if row.len() > 7 {
                    cell_i64(&row[7]) as i32
                } else {
                    0
                },
                mcp_access: 0,
                create_time: cell_dt(&row[5])?,
                update_time: cell_dt(&row[6])?,
                deleted_at: None,
            });
        }
    }

    if let Ok(range) = wb.worksheet_range("note_tags") {
        for row in range.rows().skip(1) {
            if row.len() < 6 {
                continue;
            }
            data.note_tags.push(note_tags::Model {
                id: cell_i64(&row[0]),
                note_id: cell_i64(&row[1]),
                tag_id: cell_i64(&row[2]),
                sort_order: cell_i64(&row[3]) as i32,
                create_time: cell_dt(&row[4])?,
                update_time: cell_dt(&row[5])?,
            });
        }
    }

    if let Ok(range) = wb.worksheet_range("note_history") {
        for row in range.rows().skip(1) {
            if row.len() < 8 {
                continue;
            }
            data.note_histories.push(note_history::Model {
                id: cell_i64(&row[0]),
                note_id: cell_i64(&row[1]),
                old_content: cell_str(&row[2]),
                new_content: cell_str(&row[3]),
                extra: cell_str(&row[4]),
                operate_type: cell_i64(&row[5]) as i32,
                operate_source: 0,
                operate_time: cell_dt(&row[6])?,
                create_time: cell_dt(&row[7])?,
            });
        }
    }

    let txn = db.begin().await?;
    clear_tables(&txn).await?;
    restore_data(&txn, &data).await?;
    txn.commit().await?;
    info!("Excel 备份导入完成: {}", path);
    Ok(())
}

fn cell_str(cell: &calamine::Data) -> String {
    match cell {
        calamine::Data::String(s) => s.clone(),
        calamine::Data::Empty => String::new(),
        other => other.to_string(),
    }
}

fn cell_i64(cell: &calamine::Data) -> i64 {
    match cell {
        calamine::Data::Float(f) => *f as i64,
        calamine::Data::Int(i) => *i,
        calamine::Data::String(s) => s.parse().unwrap_or(0),
        _ => 0,
    }
}

fn cell_dt(cell: &calamine::Data) -> anyhow::Result<NaiveDateTime> {
    parse_dt(&cell_str(cell))
}

// ============================================================================
// CSV 导出/导入（ZIP 打包）
// ============================================================================

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
        zip.write_all(&wtr.into_inner()?)?;
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
        zip.write_all(&wtr.into_inner()?)?;
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
        zip.write_all(&wtr.into_inner()?)?;
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
        zip.write_all(&wtr.into_inner()?)?;
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
        zip.write_all(&wtr.into_inner()?)?;
    }

    zip.finish()?;
    info!("CSV 备份导出完成: {}", path);
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
                id: r[0].parse()?,
                parent_id: r[1].parse()?,
                name: r[2].to_string(),
                description: r[3].to_string(),
                icon: r[4].to_string(),
                cls: r[5].to_string(),
                sort_order: r[6].parse()?,
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
                id: r[0].parse()?,
                name: r[1].to_string(),
                icon: r[2].to_string(),
                cls: r[3].to_string(),
                sort_order: r[4].parse()?,
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
                id: r[0].parse()?,
                notebook_id: r[1].parse()?,
                title: r[2].to_string(),
                content: r[3].to_string(),
                content_type: r[4].parse()?,
                is_pinned: if r.len() > 7 {
                    r[7].parse().unwrap_or(0)
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
                id: r[0].parse()?,
                note_id: r[1].parse()?,
                tag_id: r[2].parse()?,
                sort_order: r[3].parse()?,
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
                id: r[0].parse()?,
                note_id: r[1].parse()?,
                old_content: r[2].to_string(),
                new_content: r[3].to_string(),
                extra: r[4].to_string(),
                operate_type: r[5].parse()?,
                operate_source: 0,
                operate_time: parse_dt(&r[6])?,
                create_time: parse_dt(&r[7])?,
            });
        }
    }

    let txn = db.begin().await?;
    clear_tables(&txn).await?;
    restore_data(&txn, &data).await?;
    txn.commit().await?;
    info!("CSV 备份导入完成: {}", path);
    Ok(())
}

// ============================================================================
// 自动备份
// ============================================================================

/// 执行自动备份，将 SQL 备份保存到 `{app_data_dir}/backups/` 目录
pub async fn auto_backup(db: &DatabaseConnection, app_data_dir: &Path) -> anyhow::Result<String> {
    let backup_dir = app_data_dir.join("backups");
    if !backup_dir.exists() {
        std::fs::create_dir_all(&backup_dir)?;
    }

    let filename = format!(
        "enote_backup_{}.sql",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
    let path = backup_dir.join(&filename);
    let path_str = path.to_string_lossy().to_string();

    export_sql(db, &path_str).await?;
    info!("自动备份完成: {}", path_str);

    Ok(filename)
}

/// 清理旧备份，保留最近 max_count 个文件
pub fn cleanup_old_backups(app_data_dir: &Path, max_count: usize) -> anyhow::Result<u32> {
    let backup_dir = app_data_dir.join("backups");
    if !backup_dir.exists() {
        return Ok(0);
    }

    let mut files: Vec<_> = std::fs::read_dir(&backup_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("enote_backup_") && n.ends_with(".sql"))
                .unwrap_or(false)
        })
        .collect();

    // 按修改时间排序（最新在前）
    files.sort_by(|a, b| {
        let ta = a.metadata().and_then(|m| m.modified()).ok();
        let tb = b.metadata().and_then(|m| m.modified()).ok();
        tb.cmp(&ta)
    });

    let mut deleted = 0u32;
    if files.len() > max_count {
        for entry in &files[max_count..] {
            if std::fs::remove_file(entry.path()).is_ok() {
                deleted += 1;
            }
        }
    }

    if deleted > 0 {
        info!("已清理 {} 个旧备份文件", deleted);
    }
    Ok(deleted)
}

/// 列出所有自动备份文件
pub fn list_backups(app_data_dir: &Path) -> anyhow::Result<Vec<(String, u64)>> {
    let backup_dir = app_data_dir.join("backups");
    if !backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files: Vec<(String, u64)> = std::fs::read_dir(&backup_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with("enote_backup_") && name.ends_with(".sql") {
                let size = e.metadata().ok().map(|m| m.len()).unwrap_or(0);
                Some((name, size))
            } else {
                None
            }
        })
        .collect();

    // 按文件名倒序（最新在前）
    files.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(files)
}
