use crate::reader::reader_services;

#[tauri::command]
pub async fn reader_get_width() -> Result<f32, String> {
    Ok(reader_services::reader_get_width().await)
}

#[tauri::command]
pub async fn reader_save_width(width: f32) -> Result<(), String> {
    reader_services::reader_save_width(width).await;
    Ok(())
}