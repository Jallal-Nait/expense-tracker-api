use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseDto {
    pub product_id: Uuid,
    pub price: f32,
    pub quantity: f32,
    pub measure: String,
}
