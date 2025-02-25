#[cfg(test)]
pub mod unittests {

    use uuid::Uuid;

    use crate::{
        category::repository::CategoryRepo,
        category_repo,
        product::{dto::ProductDto, model::ProductModel, repository::ProductRepo},
        product_repo,
        tests::category::unittests::{delete as delete_category, store},
    };

    pub async fn store_without_category(repo: &ProductRepo) -> ProductModel {
        let payload = ProductDto {
            name: String::from("ORANGE"),
            category_id: None,
        };

        let exists = repo.name_exists(&payload.name, None).await.unwrap();
        assert_eq!(exists, false);

        let record = repo.store(&payload).await.unwrap();
        assert_eq!(record.name, payload.name);

        record
    }

    pub async fn store_with_category(
        repo: &ProductRepo,
        category_repo: &CategoryRepo,
    ) -> ProductModel {
        let category = store(&category_repo).await;
        let payload = ProductDto {
            name: String::from("KIWI"),
            category_id: Some(category.id),
        };

        let exists = repo.name_exists(&payload.name, None).await.unwrap();
        assert!(!exists);

        let record = repo.store(&payload).await.unwrap();
        assert_eq!(record.name, payload.name);

        record
    }

    pub async fn update(repo: &ProductRepo, record_id: &Uuid) {
        let record_db = repo.get_by_id(&record_id).await.unwrap();
        assert_eq!(record_db.id.to_string(), record_id.to_string());

        let category_id = record_db.category_id;

        let payload = ProductDto {
            name: String::from("APPLE"),
            category_id: category_id.clone(),
        };

        let exists = repo.name_exists(&payload.name, Some(&record_db.id)).await.unwrap();
        assert!(!exists);

        let record = repo.update(&payload, &record_db.id).await.unwrap();
        assert_eq!(record.name, payload.name);
    }

    pub async fn delete(repo: &ProductRepo, record_id: &Uuid) -> bool {
        let record_db = repo.get_by_id(&record_id).await.unwrap();
        assert_eq!(record_db.id.to_string(), record_id.to_string());

        let deleted = repo.delete(&record_id).await.unwrap();
        assert!(deleted);

        deleted
    }

    #[tokio::test]
    pub async fn test_crud_operations() {
        let repo = product_repo().await;
        let category_repo = category_repo().await;
        let record = store_without_category(&repo).await;
        update(&repo, &record.id).await;
        delete(&repo, &record.id).await;

        let record = store_with_category(&repo, &category_repo).await;
        delete(&repo, &record.id).await;

        match record.category_id {
            Some(c_id) => {
                delete_category(&category_repo, &c_id).await;
            }
            None => {}
        }
    }
}
