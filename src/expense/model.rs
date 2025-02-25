use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ExpenseModel {
    pub id: Uuid,
    pub product_id: Uuid,
    pub price: f32,
    pub quantity: f32,
    pub measure: String,
    pub expense_date: chrono::NaiveDate,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ExpenseProductModel {
    pub id: Uuid,
    pub product_id: Uuid,
    pub product_name: Uuid,
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub price: f32,
    pub quantity: f32,
    pub measure: String,
    pub expense_date: chrono::NaiveDate,
}
