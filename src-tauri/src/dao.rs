use sqlx::{Acquire, query, QueryBuilder, Row, Sqlite, Transaction};


use sqlx::sqlite::SqlitePool;
use tokio::join;
use common::Result;
use crate::common;
use crate::dao::entity::MangaInfo;
use crate::dao::init::{get_pool};


pub mod init;
pub mod entity;

const BIND_LIMIT: usize = 1000;

pub(crate) async fn add_comic(manga: &MangaInfo, path: Vec<String>) -> Result<()> {
    println!("POOL: {:?}", get_pool());
    let mut tx = get_pool().begin().await.expect("");

    // Insert the task, then obtain the ID of this row
    let row_id = query!(
        r#"
insert into manga_info (parent_id,
                        title,
                        cover_path,
                        read_process,
                        type_code,
                        sort)
    values (?,?,?,?,?,?);
        "#,
        manga.parent_id,
        manga.title,
        manga.cover_path,
        manga.read_process,
        manga.type_code,
        manga.sort
    )
        .execute(tx.acquire().await?)
        .await?
        .last_insert_rowid()
        ;

    tx = add_path(tx, path, row_id).await?;

    tx.commit().await?;
    Ok(())
}


pub(crate) async fn get_store_comic() -> Result<Vec<MangaInfo>> {
        let recs = sqlx::query_as(
        r#"
SELECT rowid id,
       parent_id,
       title,
       cover_path,
       read_process,
       type_code,
       sort,
       create_time,
       update_time,
       access_time
FROM manga_info
ORDER BY id desc
        "#
    )
        .fetch_all(&get_pool())
        .await?;

    Ok(recs)

}

pub(crate) async fn comic_delete(id: i64) -> Result<()> {

    let mut tx = get_pool().begin().await?;
    sqlx::query(
        r#"
delete
from manga_info
where rowid = ?;
        "#
    )
        .bind(id)
        .execute(tx.acquire().await?)
        .await?;

    sqlx::query(
        r#"
delete
from path_list
where manga_id = ?;
        "#
    )
        .bind(id)
        .execute(tx.acquire().await?)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub(crate) async fn get_path_list(id: i64) -> Result<(MangaInfo, Vec<String>)> {
    let pool = &get_pool();
    let recs_fu = sqlx::query_as(
        r#"
SELECT rowid id,
       parent_id,
       title,
       cover_path,
       read_process,
       type_code,
       sort,
       create_time,
       update_time,
       access_time
FROM manga_info
WHERE rowid = ?
        "#
    )
        .bind(id)
        .fetch_one(pool);

    let path_list_fu = sqlx::query_as(
        r#"
SELECT path
FROM path_list
WHERE manga_id = ?
        "#
    )
        .bind(id)
        .fetch_all(pool);

    let (recs, path_list) = join!(recs_fu, path_list_fu);
    let n: Vec<(String, )> = path_list?;

    println!("path: {:?}", n);

    Ok((recs?, n.into_iter().map(|p| p.0).collect()))
}

async fn add_path<'a>(mut tx: Transaction<'a, Sqlite>, paths: Vec<String>, row_id: i64) -> Result<Transaction<'a, Sqlite>> {
    for paths_chunk in paths.chunks(BIND_LIMIT) {
        QueryBuilder::new("insert into path_list (path, manga_id) ")
            .push_values(paths_chunk, |mut b, user| {
                // If you wanted to bind these by-reference instead of by-value,
                // you'd need an iterator that yields references that live as long as `query_builder`,
                // e.g. collect it to a `Vec` first.
                b.push_bind(user)
                    .push_bind(row_id);
            })
            .build()
            .execute(tx.acquire().await?)
            .await
            .expect("储存路径失败");
    };

    Ok(tx)
}

async fn complete_todo(pool: &SqlitePool, id: i64) -> Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
create table MangaInfo
(
    parent_id   integer,
    id          integer
        constraint manga_info_pk
            primary key,
    title       integer,
    path        integer,
    column_name integer
)
        "#
    )
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

async fn list_todos(pool: &SqlitePool) -> Result<()> {
//     let recs = sqlx::query!(
//         r#"
// SELECT id, description, done
// FROM todos
// ORDER BY id
//         "#
//     )
//         .fetch_all(pool)
//         .await?;
//
//     for rec in recs {
//         println!(
//             "- [{}] {}: {}",
//             if rec.done { "x" } else { " " },
//             rec.id,
//             &rec.description,
//         );
//     }
//
    Ok(())
}