use crate::dao::entity::MangaInfo;
use crate::services;

#[tauri::command]
pub async fn add_comic() -> Result<Option<MangaInfo>, String> {
    Ok(services::add_comic().await)
}
