use sqlx::{query, sqlite::SqlitePool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite:database.sqlite3").await?;
    let result = query!("SELECT * FROM person").fetch_all(&pool).await?;
    for row in result {
        println!("{:#?}", row);
    }
    Ok(())
}
