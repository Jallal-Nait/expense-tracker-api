use category::repository::CategoryRepo;
use config::AppConfig;
use product::repository::ProductRepo;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod category;
pub mod config;
pub mod enums;
pub mod expense;
pub mod product;
pub mod tests;

pub async fn init_db() -> PgPool {
    let conf = AppConfig::init();
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&conf.database_url)
        .await
        .expect("Could not connect to DATABASE_URL")
}

pub async fn category_repo() -> CategoryRepo {
    let db = init_db().await;
    CategoryRepo { db }
}

pub async fn product_repo() -> ProductRepo {
    let db = init_db().await;
    ProductRepo { db }
}
