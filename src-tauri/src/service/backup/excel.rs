use chrono::NaiveDateTime;
use sea_orm::*;
use tracing::info;

use super::{clear_tables, format_dt, parse_dt, restore_data, BackupData, BATCH_SIZE};
use crate::entity::{note, note_history, note_tags, notebook, tag};

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
    info!("Excel backup export completed: {}", path);
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
                is_starred: if row.len() > 8 {
                    cell_i64(&row[8]) as i32
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
    info!("Excel backup import completed: {}", path);
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
    let s = cell_str(cell);
    parse_dt(&s).map_err(|e| anyhow::anyhow!("Excel date parse error (cell={:?}): {}", cell, e))
}
