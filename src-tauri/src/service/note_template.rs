//! 笔记模板服务模块
//!
//! 本模块提供笔记模板相关的业务逻辑实现。
//!
//! # 功能概述
//! - 查询所有模板（按排序值和更新时间排序）
//! - 创建新模板
//! - 更新模板（智能检测变更）
//! - 删除模板

use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait, IntoActiveModel, QueryOrder,
};

use crate::{entity, model::NoteTemplate};

/// 查询所有模板
///
/// # 参数
/// - `db`: 数据库连接
///
/// # 返回
/// - `Ok(Vec<NoteTemplate>)`: 模板列表，按排序值降序、更新时间降序排列
/// - `Err`: 查询失败
pub async fn find_all(db: &DatabaseConnection) -> anyhow::Result<Vec<NoteTemplate>> {
    let templates = entity::note_template::Entity::find()
        .order_by_desc(entity::note_template::Column::SortOrder)
        .order_by_desc(entity::note_template::Column::UpdateTime)
        .all(db)
        .await?
        .into_iter()
        .map(NoteTemplate::from)
        .collect::<Vec<NoteTemplate>>();

    Ok(templates)
}

/// 创建模板
///
/// # 参数
/// - `db`: 数据库连接
/// - `template`: 模板数据
///
/// # 返回
/// - `Ok(Some(NoteTemplate))`: 创建成功，返回新模板
/// - `Err`: 创建失败
pub async fn create(
    db: &DatabaseConnection,
    template: &NoteTemplate,
) -> anyhow::Result<Option<NoteTemplate>> {
    let now = Local::now().naive_local();

    let active_model = entity::note_template::ActiveModel {
        id: NotSet,
        name: Set(template.name.clone()),
        content: Set(template.content.clone()),
        sort_order: Set(template.sort_order),
        create_time: Set(now),
        update_time: Set(now),
    };

    let entity = active_model.insert(db).await?;

    Ok(Some(NoteTemplate::from(entity)))
}

/// 根据 ID 删除模板
///
/// # 参数
/// - `db`: 数据库连接
/// - `id`: 模板 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err`: 删除失败
pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    entity::note_template::Entity::delete_by_id(id)
        .exec(db)
        .await?;

    Ok(())
}

/// 更新模板
///
/// # 参数
/// - `db`: 数据库连接
/// - `template`: 包含更新数据的模板对象
///
/// # 返回
/// - `Ok(Some(NoteTemplate))`: 更新成功，返回更新后的模板
/// - `Ok(None)`: 模板不存在
/// - `Err`: 更新失败
pub async fn update(
    db: &DatabaseConnection,
    template: &NoteTemplate,
) -> anyhow::Result<Option<NoteTemplate>> {
    if let Some(entity) = entity::note_template::Entity::find_by_id(template.id)
        .one(db)
        .await?
    {
        let mut m = template.clone();
        let mut active_model: entity::note_template::ActiveModel = entity.into_active_model();

        active_model.name.set_if_not_equals(template.name.clone());
        active_model
            .content
            .set_if_not_equals(template.content.clone());
        active_model
            .sort_order
            .set_if_not_equals(template.sort_order);

        if active_model.is_changed() {
            let now = Local::now().naive_local();

            active_model.update_time = Set(now);

            active_model.update(db).await?;

            m.update_time = Some(now);
        }

        Ok(Some(m))
    } else {
        Ok(None)
    }
}
