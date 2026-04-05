use super::*;

/// 获取所有标签
#[tauri::command]
pub async fn find_all_tags(
    app_state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<Tag>, AppError> {
    if is_server_backend(&app_state).await {
        let client = require_server(&app_state).await?;
        return client.find_all_tags().await;
    }
    let db = require_db(&app_state).await?;
    let tags = service::tag::find_all(&db).await.map_err(AppError::from)?;
    Ok(tags)
}

/// 创建标签
#[tauri::command]
pub async fn create_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    tag.validate().map_err(AppError::from)?;
    if is_server_backend(&app_state).await {
        let client = require_server(&app_state).await?;
        return client.create_tag(&tag).await;
    }
    let db = require_db(&app_state).await?;
    let result = service::tag::create(&db, &tag).await.map_err(AppError::from)?;
    if let Some(ref t) = result {
        let _ = service::app_log::log_action(
            &db, "tag", "create",
            Some(&t.id.to_string()), Some(&t.name),
            &format!("Created tag: {}", t.name), None,
        ).await;
    }
    Ok(result)
}

/// 根据 ID 删除标签
#[tauri::command]
pub async fn delete_tag_by_id(
    app_state: tauri::State<'_, Arc<AppState>>,
    id: i64,
) -> Result<(), AppError> {
    if is_server_backend(&app_state).await {
        let client = require_server(&app_state).await?;
        return client.delete_tag_by_id(id).await;
    }
    let db = require_db(&app_state).await?;
    service::tag::delete_by_id(&db, id)
        .await
        .map_err(AppError::from)?;
    let _ = service::app_log::log_action(
        &db, "tag", "delete",
        Some(&id.to_string()), None,
        &format!("Deleted tag id={}", id), None,
    ).await;
    Ok(())
}

/// 更新标签
#[tauri::command]
pub async fn update_tag(
    app_state: tauri::State<'_, Arc<AppState>>,
    tag: Tag,
) -> Result<Option<Tag>, AppError> {
    tag.validate().map_err(AppError::from)?;
    if is_server_backend(&app_state).await {
        let client = require_server(&app_state).await?;
        return client.update_tag(&tag).await;
    }
    let db = require_db(&app_state).await?;
    let result = service::tag::update(&db, &tag).await.map_err(AppError::from)?;
    if result.is_some() {
        let _ = service::app_log::log_action(
            &db, "tag", "update",
            Some(&tag.id.to_string()), Some(&tag.name),
            &format!("Updated tag: {}", tag.name), None,
        ).await;
    }
    Ok(result)
}
