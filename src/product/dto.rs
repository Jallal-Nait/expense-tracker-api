use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ProductDto {
    pub category_id: Option<Uuid>,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Product name must contain between 3 and 50 characters"
    ))]
    pub name: String,
}
