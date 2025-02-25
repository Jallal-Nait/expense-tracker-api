use sqlx::PgPool;
use uuid::Uuid;

use super::{dto::CategoryDto, model::CategoryModel};

#[derive(Debug, Clone)]
pub struct CategoryRepo {
    pub db: PgPool,
}

impl CategoryRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<CategoryModel>, sqlx::Error> {
        let sql = r#"SELECT id, name FROM categories"#;
        sqlx::query_as::<_, CategoryModel>(sql)
            .fetch_all(&self.db)
            .await
    }

    pub async fn get_by_id(&self, record_id: &Uuid) -> Result<CategoryModel, sqlx::Error> {
        let sql = r#"SELECT id, name FROM categories WHERE id=$1"#;
        sqlx::query_as::<_, CategoryModel>(sql)
            .bind(record_id)
            .fetch_one(&self.db)
            .await
    }

    pub async fn name_exists(
        &self,
        name: &str,
        record_id: Option<&Uuid>,
    ) -> Result<bool, sqlx::Error> {
        let sql = if let Some(_) = record_id {
            r#"SELECT id, name FROM categories WHERE name=$1 AND id!=$2"#
        } else {
            r#"SELECT id, name FROM categories WHERE name=$1"#
        };

        let query = sqlx::query_as::<_, CategoryModel>(sql).bind(name);
        let query = if let Some(id) = record_id {
            query.bind(id)
        } else {
            query
        };

        let record = query.fetch_optional(&self.db).await?;

        Ok(record.is_some())
    }

    pub async fn store(&self, payload: &CategoryDto) -> Result<CategoryModel, sqlx::Error> {
        let sql = r#"INSERT INTO categories (name) VALUES ($1) RETURNING id, name"#;
        sqlx::query_as::<_, CategoryModel>(sql)
            .bind(&payload.name)
            .fetch_one(&self.db)
            .await
    }

    pub async fn update(
        &self,
        payload: &CategoryDto,
        record_id: &Uuid,
    ) -> Result<CategoryModel, sqlx::Error> {
        let sql = r#"UPDATE categories SET name=$1 WHERE id=$2 RETURNING id, name"#;
        sqlx::query_as::<_, CategoryModel>(sql)
            .bind(&payload.name)
            .bind(&record_id)
            .fetch_one(&self.db)
            .await
    }

    pub async fn delete(&self, record_id: &Uuid) -> Result<bool, sqlx::Error> {
        let sql = r#"DELETE FROM categories WHERE id=$1"#;
        let record = sqlx::query(sql).bind(&record_id).execute(&self.db).await?;

        Ok(record.rows_affected() > 0)
    }
}
