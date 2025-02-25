#[cfg(test)]
pub mod unittests {
    use crate::{
        category_repo,
        product_repo,
        tests::product::unittests::{store_with_category, store_without_category},
    };

    // pub async fn store_expense(repo: &ExpenseRepo, product_repo: &ProductRepo) -> ExpenseModel {
    //     let product = product_repo.store(payload);
    // }

    #[tokio::test]
    pub async fn test_joins() {
        let category_repo = category_repo().await;
        let product_repo = product_repo().await;

        store_with_category(&product_repo, &category_repo).await;
        store_without_category(&product_repo).await;
    }
}
