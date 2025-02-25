use std::env;

use dotenv::dotenv;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("You must set DATABASE_URL in .env file");

        Self { database_url }
    }
}
