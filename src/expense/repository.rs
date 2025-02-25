use sqlx::PgPool;
use uuid::Uuid;

use super::{
    dto::ExpenseDto,
    model::{ExpenseModel, ExpenseProductModel},
};

pub struct ExpenseRepo {
    pub db: PgPool,
}

impl ExpenseRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<ExpenseProductModel>, sqlx::Error> {
        let sql = r#"
            SELECT e.id, e.product_id, e.price, e.quantity, e.measure, p.name, p.category_id, c.name
            FROM expenses e
            LEFT JOIN products p ON p.id = e.product_id
            LEFT JOIN categories c ON c.id = p.category_id 
        "#;
        sqlx::query_as::<_, ExpenseProductModel>(sql)
            .fetch_all(&self.db)
            .await
    }

    pub async fn get_by_id(&self, record_id: &Uuid) -> Result<ExpenseProductModel, sqlx::Error> {
        let sql = r#"
            SELECT e.id, e.product_id, e.price, e.quantity, e.measure, p.name, p.category_id, c.name
            FROM expenses e
            LEFT JOIN products p ON p.id = e.product_id
            LEFT JOIN categories c ON c.id = p.category_id 
            WHERE e.id=$1
        "#;
        sqlx::query_as::<_, ExpenseProductModel>(sql)
            .bind(&record_id)
            .fetch_one(&self.db)
            .await
    }

    pub async fn store(&self, payload: &ExpenseDto) -> Result<ExpenseModel, sqlx::Error> {
        let sql = r#"INSERT INTO expenses (product_id, price, quantity, measure) VALUES ($1, $2, $3, $4) RETURNING *"#;
        sqlx::query_as::<_, ExpenseModel>(sql)
            .bind(&payload.product_id)
            .bind(&payload.price)
            .bind(&payload.quantity)
            .bind(&payload.measure)
            .fetch_one(&self.db)
            .await
    }

    pub async fn update(
        &self,
        payload: &ExpenseDto,
        record_id: &Uuid,
    ) -> Result<ExpenseModel, sqlx::Error> {
        let sql = r#"UPDATE expenses SET product_id=$1, price=$2, quantity=$3, measure=$4 WHERE id=$5 RETURNING *"#;
        sqlx::query_as::<_, ExpenseModel>(sql)
            .bind(&payload.product_id)
            .bind(&payload.price)
            .bind(&payload.quantity)
            .bind(&payload.measure)
            .bind(&record_id)
            .fetch_one(&self.db)
            .await
    }

    pub async fn delete(&self, record_id: &Uuid) -> Result<bool, sqlx::Error> {
        let sql = r#"DELETE FROM expenses WHERE id=$1"#;
        let record = sqlx::query(sql).bind(&record_id).execute(&self.db).await?;

        Ok(record.rows_affected() > 0)
    }
}
