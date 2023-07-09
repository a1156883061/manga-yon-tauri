use sqlx::{Acquire, query, QueryBuilder, Sqlite, Transaction};


use sqlx::sqlite::SqlitePool;
use common::Result;
use crate::common;
use crate::dao::entity::MangaInfo;
use crate::dao::init::{get_pool};


pub mod init;
pub mod entity;

const BIND_LIMIT: usize = 1000;

pub(crate) async fn add_comic(manga: &MangaInfo, path: Vec<String>) -> Result<()> {
    println!("POOL: {:?}", get_pool());
    // let mut connection = get_pool().acquire().await?;
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
        .execute(&mut tx)
        .await?
        .last_insert_rowid()
        ;

    tx = add_path(tx, path, row_id).await?;

    tx.commit().await?;
    Ok(())
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