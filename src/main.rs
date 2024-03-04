use crate::repo::ExDate;
use repo::CreateItem;
use repo::Item;
use sqlx::PgPool;
use std::env;

mod repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&url).await?;

    let ex_date = ExDate {
        year: 2024,
        month: 5,
        day: 25,
    };

    let payload = CreateItem {
        name: "potato".to_string(),
        expiration_date: ex_date,
    };

    let put = insert(&pool, payload).await?;
    println!("{:#?}", put);

    let get = select(&pool, 1).await?;
    println!("{:#?}", get);

    Ok(())
}

async fn insert(pool: &PgPool, payload: CreateItem) -> Result<Item, Box<dyn std::error::Error>> {
    let item = sqlx::query_as::<_, Item>(
        r#"
INSERT INTO item (name, year, month, day)
VALUES ($1, $2, $3, $4)
RETURNING *
    "#,
    )
    .bind(payload.name)
    .bind(payload.expiration_date.year)
    .bind(payload.expiration_date.month)
    .bind(payload.expiration_date.day)
    .fetch_one(pool)
    .await?;

    Ok(item)
}

async fn select(pool: &PgPool, id: i32) -> Result<Item, Box<dyn std::error::Error>> {
    let item = sqlx::query_as::<_, Item>(
        r#"
SELECT * FROM item
WHERE id = $1
    "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .expect("select error");

    Ok(item)
}

// !!!!!以下のコードは失敗したもの!!!!
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>>{
//     dotenvy::dotenv()?;
//     let url = env::var("DATABASE_URL")?;
//     let pool = PgPool::connect(&url).await?;

//     let payload = CreateItem {
//         name: "じゃがいも".to_string(),
//         year: 2025,
//         month: 4,
//         day: 23,
//     };

//     let res = insert(&pool, payload).await?;
//     println!("{:?}", res);

//     sleep(time::Duration::from_secs(5)).await;

//     let res2 = select(&pool, 1).await?;
//     println!("{:?}", res2);

//     Ok(())
// }

// async fn insert(pool: &PgPool, payload: CreateItem) -> Result<Item, Box<dyn std::error::Error>> {
//     let item = sqlx::query_as::<_, Item>(r#"
// INSERT INTO item (name, year, month, day)
// VALUES ($1, $2, $3, $4)
// RETURNING *
//     "#)
//     .bind(payload.name)
//     .bind(payload.year)
//     .bind(payload.month)
//     .bind(payload.day)
//     .fetch_one(pool)
//     .await?;

//     Ok(item)
// }

// async fn select(pool: &PgPool, id: i32) -> Result<Item, Box<dyn std::error::Error>> {
//     let item = sqlx::query_as::<_, Item>(r#"
// SELECT * FROM item
// WHERE id = $1
//     "#)
//     .bind(id)
//     .fetch_one(pool)
//     .await.expect("select error");

//     Ok(item)
// }
