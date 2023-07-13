use std::{env};

use lazy_static::lazy_static;
use sqlx::migrate::{MigrateDatabase};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use tokio::sync::OnceCell;
use crate::common::Result;

// pub static POOL: OnceCell<Pool<Sqlite>> = Lazy::new(|| {
//     let mut m = HashMap::new();
//     m.insert(13, "Spica".to_string());
//     m.insert(74, "Hoyten".to_string());
//     Mutex::new(m)
// });
lazy_static! {
    pub static ref POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();
}

pub async fn init_pool() {
    println!("test");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("未设置数据库连接的环境变量"))
        .await.expect("");
    POOL.set(pool).expect("初始化错误");
}

pub fn get_pool() -> Pool<Sqlite> {
    POOL.get().unwrap().clone()
}

pub async fn init_database() -> Result<()> {
    // TODO 生产环境，放入user.dir
    let db_url = &env::var("DATABASE_URL").expect("未设置数据库连接的环境变量");
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        };
    } else {
        println!("Database already exists");
    }
    Ok(())
}

pub async fn migrate() -> Result<()>{

    let mut con = POOL.get().unwrap().acquire().await?;
    sqlx::migrate!().run(&mut con).await?;
    Ok(())
}
