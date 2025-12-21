//! 笔记本服务模块
//!
//! 本模块提供笔记本相关的业务逻辑实现。
//!
//! # 功能概述
//! - 查询所有笔记本（按排序值和更新时间排序）
//! - 创建新笔记本
//! - 更新笔记本（智能检测变更）
//! - 删除笔记本

use chrono::Local;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    DatabaseConnection, EntityTrait, IntoActiveModel, QueryOrder,
};

use crate::{entity, model::Notebook};

/// 查询所有笔记本
///
/// # 参数
/// - `db`: 数据库连接
///
/// # 返回
/// - `Ok(Vec<Notebook>)`: 笔记本列表，按排序值降序、更新时间降序排列
/// - `Err`: 查询失败
pub async fn find_all(db: &DatabaseConnection) -> anyhow::Result<Vec<Notebook>> {
    let notebooks = entity::notebook::Entity::find()
        .order_by_desc(entity::notebook::Column::SortOrder)
        .order_by_desc(entity::notebook::Column::UpdateTime)
        .all(db)
        .await?
        .iter()
        .map(Notebook::from)
        .collect::<Vec<Notebook>>();

    Ok(notebooks)
}

/// 创建笔记本
///
/// # 参数
/// - `db`: 数据库连接
/// - `notebook`: 笔记本数据
///
/// # 返回
/// - `Ok(Some(Notebook))`: 创建成功，返回新笔记本
/// - `Err`: 创建失败
///
/// # 说明
/// - 自动设置创建时间和更新时间为当前时间
/// - ID 由数据库自动生成
pub async fn create(
    db: &DatabaseConnection,
    notebook: &Notebook,
) -> anyhow::Result<Option<Notebook>> {
    let now = Local::now().naive_local();

    let active_model = entity::notebook::ActiveModel {
        id: NotSet,
        parent_id: Set(notebook.parent_id),
        name: Set(notebook.name.clone()),
        description: Set(notebook.description.clone()),
        icon: Set(notebook.icon.clone()),
        cls: Set(notebook.cls.clone()),
        sort_order: Set(notebook.sort_order),
        create_time: Set(now),
        update_time: Set(now),
    };

    let entity = active_model.insert(db).await?;

    Ok(Some(Notebook::from(entity)))
}

/// 根据 ID 删除笔记本
///
/// # 参数
/// - `db`: 数据库连接
/// - `id`: 笔记本 ID
///
/// # 返回
/// - `Ok(())`: 删除成功
/// - `Err`: 删除失败
///
/// # 注意
/// 删除笔记本不会自动删除其下的笔记，需要前端或业务层处理
pub async fn delete_by_id(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
    entity::notebook::Entity::delete_by_id(id).exec(db).await?;

    Ok(())
}

/// 更新笔记本
///
/// # 参数
/// - `db`: 数据库连接
/// - `notebook`: 包含更新数据的笔记本对象
///
/// # 返回
/// - `Ok(Some(Notebook))`: 更新成功，返回更新后的笔记本
/// - `Ok(None)`: 笔记本不存在
/// - `Err`: 更新失败
///
/// # 智能更新
/// - 使用 `set_if_not_equals` 仅更新有变化的字段
/// - 只有实际发生变更时才更新 update_time
pub async fn update(
    db: &DatabaseConnection,
    notebook: &Notebook,
) -> anyhow::Result<Option<Notebook>> {
    if let Some(entity) = entity::notebook::Entity::find_by_id(notebook.id)
        .one(db)
        .await?
    {
        let mut m = notebook.clone();
        let mut active_model: entity::notebook::ActiveModel = entity.into_active_model();

        // 使用 set_if_not_equals 仅更新有变化的字段
        active_model.name.set_if_not_equals(notebook.name.clone());
        active_model
            .description
            .set_if_not_equals(notebook.description.clone());
        active_model.icon.set_if_not_equals(notebook.icon.clone());
        active_model.cls.set_if_not_equals(notebook.cls.clone());
        active_model
            .sort_order
            .set_if_not_equals(notebook.sort_order);

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
