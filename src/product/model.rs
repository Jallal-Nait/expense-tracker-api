use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProductModel {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProductCategoryModel {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub name: String,
}
