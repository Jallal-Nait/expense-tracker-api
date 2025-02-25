#[cfg(test)]
pub mod unittests {
    use uuid::Uuid;

    use crate::{
        category::{dto::CategoryDto, model::CategoryModel, repository::CategoryRepo},
        category_repo,
    };

    pub async fn store(repo: &CategoryRepo) -> CategoryModel {
        let payload = CategoryDto {
            name: String::from("FRUITS"),
        };

        let exists = repo.name_exists(&payload.name, None).await.unwrap();
        assert!(!exists);

        let record = repo.store(&payload).await.unwrap();
        assert_eq!(record.name, payload.name);

        record
    }

    async fn get_all(repo: &CategoryRepo) {
        let records = repo.get_all().await.unwrap();
        assert!(records.len() > 0);
    }

    async fn update(repo: &CategoryRepo, record_id: &Uuid) {
        let record_db = repo.get_by_id(&record_id).await.unwrap();
        assert_eq!(record_db.id.to_string(), record_id.to_string());

        let payload = CategoryDto {
            name: String::from("FRUITS & DESSERTS"),
        };
        let exists = repo
            .name_exists(&payload.name, Some(&record_id))
            .await
            .unwrap();
        assert!(!exists);

        let record = repo.update(&payload, &record_id).await.unwrap();
        assert_eq!(record.name, payload.name);
    }

    pub async fn delete(repo: &CategoryRepo, record_id: &Uuid) -> bool {
        let record_db = repo.get_by_id(&record_id).await.unwrap();
        assert_eq!(record_db.id.to_string(), record_id.to_string());

        let deleted = repo.delete(&record_id).await.unwrap();
        assert!(deleted);

        deleted
    }

    #[tokio::test]
    pub async fn test_crud_operations() {
        let repo = category_repo().await;
        let category = store(&repo).await;
        get_all(&repo).await;
        update(&repo, &category.id).await;
        delete(&repo, &category.id).await;
    }
}
