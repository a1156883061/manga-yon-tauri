
use crate::reader::reader_dao;
use crate::common::Result;

///
/// 获取宽度
///
pub async fn reader_get_width() -> f32 {
    reader_dao::reader_get_width().await
}

///
/// 保存宽度
///
pub async fn reader_save_width(width: f32) {
    reader_dao::reader_save_width(width).await
}
///
/// 保存宽度
///
pub async fn reader_save_read_process(id: i64, process: i32) -> Result<()> {
    reader_dao::reader_save_read_process(id, process).await
}