use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::task::{Context, Poll};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use tauri::api::dialog::FileDialogBuilder;
use tree_magic::from_filepath;
use crate::common::Result;
use crate::dao;
use crate::dao::entity::MangaInfo;
use crate::services::file_filter::FILTER_ARR;

pub mod file_filter;


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
            let mut file_dialog_builder = FileDialogBuilder::new();
            for file_filter in FILTER_ARR {
                file_dialog_builder = file_dialog_builder.add_filter(file_filter.name(), file_filter.extensions());
            }
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
                        let image_info = build_image_info(file_list, dir_name);
                        *arc.lock().unwrap() = Some(Some(image_info));
                    }
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

fn get_file_name_str(path_next: &Path) -> &str {
    path_next.file_name().unwrap().to_str().unwrap()
}

fn build_image_info(file_list: Vec<PathBuf>, dir_name: String) -> ImageInfo {
    let file_name_list = get_image_file_name_list(file_list);
    ImageInfo(dir_name, file_name_list)
}

///
/// 过滤非图片文件
/// 按照自然顺序排序文件名 [2, 04 , 3] -> [2, 3, 04]
/// 转换为文件路径string
///
fn get_image_file_name_list(file_list: Vec<PathBuf>) -> Vec<String> {
    let mut file_list: Vec<&PathBuf> = file_list
        .iter()
        .filter(|path| path.is_file())
        .filter(|each_file_path| {
            let mime_type = from_filepath(each_file_path.as_path());
            mime_type.starts_with("image/")
        }).collect();
    file_list
        .sort_by(|path_prev, path_next| natord::compare(get_file_name_str(path_prev)
                                                        , get_file_name_str(path_next)));
    let file_name_list: Vec<String> = file_list
        .iter()
        .map(|each_file| each_file.to_string_lossy().to_string())
        .collect();
    file_name_list
}

fn get_dir_name(path: &Path) -> String {
    path.parent()
        .map(Path::file_name)
        .unwrap()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

/// 1. 添加图片列表
/// 2. 选择单一图片时，获取其他图片
pub async fn add_comic() -> Option<MangaInfo> {
    let option = match PickFileFuture::new().await.take() {
        None => {
            None
        }
        Some(image_info) => {
            let manga_info = add_and_get_manga_info(image_info).await;
            Some(manga_info)
        }
    };
    println!("option: {:?}", option);
    option
}

async fn add_and_get_manga_info(image_info: ImageInfo) -> MangaInfo {
    let mut manga_info = MangaInfo {
        id: None,
        parent_id: 0,
        title: image_info.0,
        cover_path: image_info.1.get(0).unwrap().to_string(),
        read_process: 0,
        type_code: "".to_string(),
        sort: 0,
    };
    let id = dao::add_comic(&manga_info, image_info.1).await.expect("TODO: panic message");
    manga_info.id = Some(id);
    manga_info
}

pub async fn add_comic_folder() -> Vec<MangaInfo> {
    let file_dialog_builder = FileDialogBuilder::new();
    let (sender, rev) = mpsc::channel();
    file_dialog_builder.pick_folders(move |folders_op| {
        match folders_op {
            None => {
                sender.send(Vec::new()).unwrap();
            }
            Some(folders) => {
                let image_infos: Vec<ImageInfo> = folders.iter()
                    .map(|folder| {
                        (folder, get_image_file_name_list(get_file_list_in_folder(folder)))
                    })
                    .filter(|(_, file_list)| !file_list.is_empty())
                    .map(|(folder, file_name_list)| {
                        let dir_name = folder.file_name().unwrap().to_string_lossy().to_string();
                        ImageInfo(dir_name, file_name_list)
                    }).collect();
                sender.send(image_infos).unwrap();
            }
        }
    });
    let image_infos = rev.recv().expect("获取错误");
    let fu: Vec<_> = image_infos.into_iter().map(|image_info| {
        add_and_get_manga_info(image_info)
    }).collect();
    join_all(fu).await
}

pub async fn get_store_comic() -> Result<Vec<MangaInfo>> {
    dao::get_store_comic().await
}

pub async fn comic_delete(id: i64) -> Result<()> {
    dao::comic_delete(id).await
}

pub async fn read_comic(handle: tauri::AppHandle, id: i64) -> Result<()> {
    tauri::WindowBuilder::new(
        &handle,
        id.to_string(), /* the unique window label */
        tauri::WindowUrl::App("reader.html".into()),
    ).build().unwrap();
    Ok(())
}

pub async fn get_path_list(id: i64) -> Result<(MangaInfo, Vec<String>)> {
    dao::get_path_list(id).await
}


fn get_file_list(file: &Path) -> Vec<PathBuf> {
    let parent_path = file.parent().unwrap();
    get_file_list_in_folder(parent_path)
}

fn get_file_list_in_folder(parent_path: &Path) -> Vec<PathBuf> {
    parent_path.read_dir().expect("读取目录出错").map(|path| {
        path.unwrap().path()
    }).collect()
}
