use anyhow::Result;
use dotenvy::EnvLoader;
use sqlx::{postgres::PgPoolOptions, query};

#[tokio::main]
async fn main() -> Result<()> {
    let dotenv = EnvLoader::new().load()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenv.var("DATABASE_URL")?)
        .await?;

    let result = query!("SELECT * FROM auth.users").fetch_all(&pool).await?;

    for row in result {
        println!("{:?}", row.email);
    }

    Ok(())
}
