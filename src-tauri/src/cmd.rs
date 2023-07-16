use crate::dao::entity::MangaInfo;
use crate::services;

#[tauri::command]
pub async fn add_comic() -> Result<Option<MangaInfo>, String> {
    Ok(services::add_comic().await)
}

#[tauri::command]
pub async fn get_store_comic() -> Result<Vec<MangaInfo>, String> {
    services::get_store_comic().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn comic_delete(id: i64) -> Result<(), String> {
    services::comic_delete(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn read_comic(handle: tauri::AppHandle, id: i64) -> Result<(), String> {
    services::read_comic(handle ,id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_comic(id: i64) -> Result<(MangaInfo, Vec<String>), String> {
    services::get_path_list(id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn add_comic_folder() -> Result<Vec<MangaInfo>, String> {
    Ok(services::add_comic_folder().await)
}
