//! 标签服务模块
//!
//! 本模块提供标签相关的业务逻辑实现。
//!
//! # 功能概述
//! - 查询所有标签（按排序值和更新时间排序）
//! - 创建新标签
//! - 更新标签（智能检测变更）
//! - 删除标签

use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
    TransactionTrait,
};

use crate::{entity, model::Tag};

/// 查询所有标签
///
/// # 参数
/// - `db`: 数据库连接
///
/// # 返回
/// - `Ok(Vec<Tag>)`: 标签列表，按排序值降序、更新时间降序排列
/// - `Err`: 查询失败
pub async fn find_all(db: &DatabaseConnection) -> anyhow::Result<Vec<Tag>> {
    let tags = entity::tag::Entity::find()
        .order_by_desc(entity::tag::Column::SortOrder)
        .order_by_desc(entity::tag::Column::UpdateTime)
        .all(db)
        .await?
        .into_iter()
        .map(Tag::from)
        .collect::<Vec<Tag>>();

    Ok(tags)
}

/// 创建标签
///
/// # 参数
/// - `db`: 数据库连接
/// - `tag`: 标签数据
///
/// # 返回
/// - `Ok(Some(Tag))`: 创建成功，返回新标签
/// - `Err`: 创建失败
///
/// # 说明
/// - 自动设置创建时间和更新时间为当前时间
/// - ID 由数据库自动生成
pub async fn create(db: &DatabaseConnection, tag: &Tag) -> anyhow::Result<Option<Tag>> {
    let now = Local::now().naive_local();

    let active_model = entity::tag::ActiveModel {
        id: NotSet,
        name: Set(tag.name.clone()),
        icon: Set(tag.icon.clone()),
        cls: Set(tag.cls.clone()),
        sort_order: Set(tag.sort_order),
        mcp_access: Set(tag.mcp_access),
        create_time: Set(now),
        update_time: Set(now),
    };

    let entity = active_model.insert(db).await?;

    Ok(Some(Tag::from(entity)))
}

/// 根据 ID 删除标签
///
/// # 参数
/// - `db`: 数据库连接
/// - `id`: 标签 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err`: 删除失败
///
/// # 注意
/// 级联删除：事务中先删除 note_tags 关联行，再删除 tag
pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    let txn = db.begin().await?;

    // Delete all note_tags associations for this tag
    entity::note_tags::Entity::delete_many()
        .filter(entity::note_tags::Column::TagId.eq(id))
        .exec(&txn)
        .await?;

    // Delete the tag itself
    entity::tag::Entity::delete_by_id(id).exec(&txn).await?;

    txn.commit().await?;

    Ok(())
}

/// 更新标签
///
/// # 参数
/// - `db`: 数据库连接
/// - `tag`: 包含更新数据的标签对象
///
/// # 返回
/// - `Ok(Some(Tag))`: 更新成功，返回更新后的标签
/// - `Ok(None)`: 标签不存在
/// - `Err`: 更新失败
///
/// # 智能更新
/// - 使用 `set_if_not_equals` 仅更新有变化的字段
/// - 只有实际发生变更时才更新 update_time
pub async fn update(db: &DatabaseConnection, tag: &Tag) -> anyhow::Result<Option<Tag>> {
    if let Some(entity) = entity::tag::Entity::find_by_id(tag.id).one(db).await? {
        let mut m = tag.clone();
        let mut active_model: entity::tag::ActiveModel = entity.into_active_model();

        // 使用 set_if_not_equals 仅更新有变化的字段
        active_model.name.set_if_not_equals(tag.name.clone());
        active_model.icon.set_if_not_equals(tag.icon.clone());
        active_model.cls.set_if_not_equals(tag.cls.clone());
        active_model.sort_order.set_if_not_equals(tag.sort_order);
        active_model.mcp_access.set_if_not_equals(tag.mcp_access);

        // 只有实际发生变更时才执行更新
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

/// 批量更新标签排序
pub async fn reorder(db: &DatabaseConnection, orders: Vec<(i64, i32)>) -> anyhow::Result<()> {
    if orders.is_empty() {
        return Ok(());
    }

    let ids: Vec<i64> = orders.iter().map(|(id, _)| *id).collect();
    let entities = entity::tag::Entity::find()
        .filter(entity::tag::Column::Id.is_in(ids))
        .all(db)
        .await?;

    let order_map: std::collections::HashMap<i64, i32> = orders.into_iter().collect();
    let now = Local::now().naive_local();

    for entity_model in entities {
        if let Some(&sort_order) = order_map.get(&entity_model.id) {
            let mut active_model: entity::tag::ActiveModel = entity_model.into_active_model();
            active_model.sort_order.set_if_not_equals(sort_order);
            if active_model.is_changed() {
                active_model.update_time = Set(now);
                active_model.update(db).await?;
            }
        }
    }
    Ok(())
}
