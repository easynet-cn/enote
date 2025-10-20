use std::sync::Arc;

use sea_orm::{EntityTrait, QueryOrder};

use crate::{config::AppState, entity, model::Notebook};

#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Notebook>, String> {
    let db = &app_state.database_connection;

    let categories = entity::notebook::Entity::find()
        .order_by_desc(entity::notebook::Column::SortOrder)
        .all(db)
        .await
        .map_err(|e| e.to_string())?
        .iter()
        .map(Notebook::from)
        .collect::<Vec<Notebook>>();

    Ok(categories)
}
