use crate::common::Result;
use crate::dao::init::get_pool;

/// 宽度
const WIDTH_CODE: i32 = 1;


pub(crate) async fn reader_get_width() -> f32 {
    // 默认值0.8宽度
    get_val(WIDTH_CODE).await
        .map_or(0.9, |val| val.parse::<f32>().expect("转换宽度数据出错"))
}

pub(crate) async fn reader_save_width(val: f32) {
    set_val(WIDTH_CODE, val).await.expect("更新失败");
}

async fn get_val(code: i32) -> Result<String> {
    let pool = &get_pool();
    let result: (String, ) = sqlx::query_as(
        r#"
SELECT val
FROM config_dict
WHERE code = ?
        "#
    )
        .bind(code)
        .fetch_one(pool).await?;
    Ok(result.0)
}

async fn set_val(code: i32, val: f32) -> Result<()> {
    let pool = &get_pool();
    sqlx::query(
        r#"
update config_dict
set val = ?
where code = ?;
        "#
    )
        .bind(val)
        .bind(code)
        .execute(pool).await?;
    Ok(())
}