use super::*;

/// 获取所有笔记本
#[tauri::command]
pub async fn find_all_notebooks(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Notebook>, AppError> {
    let db = require_db(&app_state).await?;
    let notebooks = service::notebook::find_all(&db)
        .await
        .map_err(AppError::from)?;
    Ok(notebooks)
}

/// 创建笔记本
#[tauri::command]
pub async fn create_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    notebook.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    let result = service::notebook::create(&db, &notebook)
        .await
        .map_err(AppError::from)?;
    if let Some(ref nb) = result {
        let _ = service::app_log::log_action(
            &db, "notebook", "create",
            Some(&nb.id.to_string()), Some(&nb.name),
            &format!("Created notebook: {}", nb.name), None,
        ).await;
    }
    Ok(result)
}

/// 根据 ID 删除笔记本
#[tauri::command]
pub async fn delete_notebook_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    let db = require_db(&app_state).await?;
    service::notebook::delete_by_id(&db, id)
        .await
        .map_err(AppError::from)?;
    let _ = service::app_log::log_action(
        &db, "notebook", "delete",
        Some(&id.to_string()), None,
        &format!("Deleted notebook id={}", id), None,
    ).await;
    Ok(())
}

/// 更新笔记本
#[tauri::command]
pub async fn update_notebook(
    app_state: tauri::State<'_, Arc<AppState>>,
    notebook: Notebook,
) -> Result<Option<Notebook>, AppError> {
    notebook.validate().map_err(AppError::from)?;
    let db = require_db(&app_state).await?;
    let result = service::notebook::update(&db, &notebook)
        .await
        .map_err(AppError::from)?;
    if result.is_some() {
        let _ = service::app_log::log_action(
            &db, "notebook", "update",
            Some(&notebook.id.to_string()), Some(&notebook.name),
            &format!("Updated notebook: {}", notebook.name), None,
        ).await;
    }
    Ok(result)
}
