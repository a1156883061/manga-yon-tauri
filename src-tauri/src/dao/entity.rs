use serde::{Deserialize, Serialize};

///
/// 漫画信息
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaInfo {
    pub id: Option<i64>,
    /// 目录id
    pub parent_id: i32,
    /// 标题
    pub title: String,
    /// 封面图片路径
    pub cover_path: String,
    /// 阅读进度，第几页，对应的图片的ID
    pub read_process: i32,
    /// 类型
    pub type_code: String,
    /// 排序
    pub sort: i32,
}