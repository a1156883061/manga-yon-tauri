use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use serde::{Deserialize, Serialize};
use tauri::api::dialog::FileDialogBuilder;
use tree_magic::{from_filepath};
use crate::common::Result;
use crate::dao;
use crate::dao::entity::MangaInfo;


#[derive(Debug, Serialize, Deserialize)]
struct ImageInfo(String, Vec<String>);


struct PickFileFuture {
    file_list: Arc<Mutex<Option<Option<ImageInfo>>>>,
}

impl PickFileFuture {
    fn new() -> PickFileFuture {
        PickFileFuture {
            file_list: Arc::new(Mutex::new(None))
        }
    }

    fn get_file_name_str(path_next: &PathBuf) -> &str {
        path_next.file_name().unwrap().to_str().unwrap()
    }
}

impl Future for PickFileFuture {
    type Output = Option<ImageInfo>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.file_list.lock().unwrap();
        if guard.is_some() {
            Poll::Ready(guard.take().unwrap())
        } else {
            let waker = cx.waker().clone();
            let arc = self.file_list.clone();
            let file_dialog_builder = FileDialogBuilder::new();
            file_dialog_builder.pick_files(move |file| {
                match file {
                    None => {
                        *arc.lock().unwrap() = Some(None);
                    }
                    Some(mut file_list) => {
                        let dir_name = get_dir_name(file_list.get(0).unwrap());
                        if file_list.len() == 1 {
                            file_list = get_file_list(file_list.get(0).unwrap());
                        }
                        let mut file_list: Vec<&PathBuf> = file_list
                            .iter()
                            .filter(|path| path.is_file())
                            .filter(|each_file_path| {
                                let mime_type = from_filepath(each_file_path.as_path());
                                mime_type.starts_with("image/")
                            }).collect();
                        file_list
                            .sort_by(|path_prev, path_next| natord::compare(Self::get_file_name_str(path_prev)
                                                                            , Self::get_file_name_str(path_next)));
                        let file_name_list: Vec<String> = file_list
                            .iter()
                            .map(|each_file| String::from(each_file.to_string_lossy().to_owned().to_string()))
                            .collect();
                        println!("file_list: {:?}", file_name_list);
                        *arc.lock().unwrap() = Some(Some(ImageInfo(dir_name, file_name_list)));
                    }
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

fn get_dir_name(path: &PathBuf) -> String {
    String::from(path.parent()
        .map(Path::file_name)
        .unwrap()
        .unwrap()
        .to_string_lossy()
        .to_owned()
        .to_string())
}

/// 1. 添加图片列表
/// 2. 选择单一图片时，获取其他图片
pub async fn add_comic() -> Option<MangaInfo> {
    let option = match PickFileFuture::new().await.take() {
        None => {
            None
        }
        Some(image_info) => {
            let manga_info = MangaInfo {
                id: None,
                parent_id: 0,
                title: image_info.0,
                cover_path: image_info.1.get(0).unwrap().to_string(),
                read_process: 0,
                type_code: "".to_string(),
                sort: 0,
            };
            let maga_info_op = Some(manga_info);
            dao::add_comic(&maga_info_op.as_ref().unwrap(), image_info.1).await.expect("TODO: panic message");
            maga_info_op
        }
    };
    println!("option: {:?}", option);
    option
}

pub async fn get_store_comic() -> Result<Vec<MangaInfo>> {
    dao::get_store_comic().await
}

pub async fn comic_delete(id: i64) -> Result<()> {
    dao::comic_delete(id).await
}

pub async fn read_comic(handle: tauri::AppHandle, id: i64) -> Result<()> {
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        id.to_string(), /* the unique window label */
        tauri::WindowUrl::App("reader.html".into())
    ).build().unwrap();
    Ok(())
}

pub async fn get_path_list(id: i64) -> Result<(MangaInfo, Vec<String>)> {
    dao::get_path_list(id).await
}



fn get_file_list(file: &PathBuf) -> Vec<PathBuf> {
    let parent_path = file.parent().unwrap();
    parent_path.read_dir().expect("读取目录出错").map(|path| {
        path.unwrap().path()
    }).collect()
}
