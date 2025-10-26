use sea_orm::{DatabaseConnection, EntityTrait, QuerySelect, prelude::Expr, sea_query::Asterisk};

use crate::entity;

pub async fn total_count(db: &DatabaseConnection) -> anyhow::Result<i64> {
    let total_count = entity::note::Entity::find()
        .select_only()
        .column_as(Expr::col(Asterisk).count(), "count")
        .into_tuple::<i64>()
        .one(db)
        .await?
        .unwrap_or_default();

    Ok(total_count)
}
