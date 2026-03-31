use std::io::Read;

use sea_orm::*;
use tracing::info;

use super::{clear_tables, format_dt, parse_dt, restore_data, BackupData, BATCH_SIZE};
use crate::entity::{note, note_history, note_tags, notebook, tag};

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
    info!("CSV backup import completed: {}", path);
    Ok(())
}
