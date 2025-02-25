use sqlx::PgPool;
use uuid::Uuid;

use super::{
    dto::ProductDto,
    model::{ProductCategoryModel, ProductModel},
};

#[derive(Debug, Clone)]
pub struct ProductRepo {
    pub db: PgPool,
}

impl ProductRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<ProductCategoryModel>, sqlx::Error> {
        let sql = r#"
            SELECT p.id, p.name, p.category_id, c.name AS category_name
            FROM products p
            LEFT JOIN categories c ON c.id = p.category_id
        "#;

        sqlx::query_as::<_, ProductCategoryModel>(sql)
            .fetch_all(&self.db)
            .await
    }

    pub async fn get_by_id(&self, record_id: &Uuid) -> Result<ProductModel, sqlx::Error> {
        let sql = r#"SELECT * FROM products WHERE id=$1"#;

        sqlx::query_as::<_, ProductModel>(sql)
            .bind(&record_id)
            .fetch_one(&self.db)
            .await
    }

    pub async fn name_exists(
        &self,
        name: &str,
        record_id: Option<&Uuid>,
    ) -> Result<bool, sqlx::Error> {
        let sql = if let Some(_) = record_id {
            r#"SELECT * FROM products WHERE name=$1 AND id!=$2"#
        } else {
            r#"SELECT * FROM products WHERE name=$1"#
        };

        let query = sqlx::query_as::<_, ProductModel>(sql).bind(name);
        let query = if let Some(id) = record_id {
            query.bind(id)
        } else {
            query
        };

        let record = query.fetch_optional(&self.db).await?;

        Ok(record.is_some())
    }

    pub async fn store(&self, payload: &ProductDto) -> Result<ProductModel, sqlx::Error> {
        let sql = r#"
            INSERT INTO products (name, category_id) VALUES ($1, $2) RETURNING *
        "#;

        sqlx::query_as::<_, ProductModel>(sql)
            .bind(&payload.name)
            .bind(&payload.category_id)
            .fetch_one(&self.db)
            .await
    }

    pub async fn update(
        &self,
        payload: &ProductDto,
        record_id: &Uuid,
    ) -> Result<ProductModel, sqlx::Error> {
        let sql = r#"UPDATE products SET name=$1, category_id=$2, updated_at=now() WHERE id=$3 RETURNING *"#;

        sqlx::query_as::<_, ProductModel>(sql)
            .bind(&payload.name)
            .bind(&payload.category_id)
            .bind(&record_id)
            .fetch_one(&self.db)
            .await
    }

    pub async fn delete(&self, record_id: &Uuid) -> Result<bool, sqlx::Error> {
        let sql = r#"DELETE FROM products WHERE id=$1"#;
        let record = sqlx::query(sql).bind(&record_id).execute(&self.db).await?;

        Ok(record.rows_affected() > 0)
    }
}
