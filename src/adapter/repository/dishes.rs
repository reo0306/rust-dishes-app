use sqlx::{Pool, MySql};
use anyhow::Result;
use ulid::Ulid;

use crate::adapter::model::dishes::{DishesTable, FetchCategoryParam, FetchCategoryDishesParam, DishesParam};
use crate::domain::model::dishes::{FetchCategory, FetchCategoryDishes, Dishes};

pub struct DishesRepository;

impl DishesRepository {
    pub async fn fetch_categories(pool: &Pool<MySql>, fc: FetchCategory) -> Result<Vec<DishesTable>> {
        let fcp: FetchCategoryParam = fc.into();

        let categories = sqlx::query_as(
            r#"
                SELECT * FROM dishes WHERE category = ?
            "#,
        )
        .bind(fcp.category)
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }

    pub async fn fetch_categories_dishes(pool: &Pool<MySql>, fcd: FetchCategoryDishes) -> Result<Vec<DishesTable>> {
        let fcdp: FetchCategoryDishesParam = fcd.into();

        let categories_dishes = sqlx::query_as(
            r#"
                SELECT * FROM dishes WHERE category = ? AND material = ?
            "#,
        )
        .bind(fcdp.category)
        .bind(fcdp.material)
        .fetch_all(pool)
        .await?;

        Ok(categories_dishes)
    }

    pub async fn find_dishes(pool: &Pool<MySql>, dishes: Dishes) -> Result<DishesTable> {
        let dp: DishesParam = dishes.into();

        let dishes = sqlx::query_as(
            r#"
                SELECT * FROM dishes WHERE category = ? AND material = ? AND name = ?
            "#,
        )
        .bind(dp.category)
        .bind(dp.material)
        .bind(dp.name)
        .fetch_one(pool)
        .await?;

        Ok(dishes)
    }

    pub async fn create_dishes(pool: &Pool<MySql>, dishes: Dishes) -> Result<()> {
        let dp: DishesParam = dishes.into();
        let ulid = Ulid::new();

        sqlx::query(
            r#"
                INSERT INTO dishes (id, category, material, name) VALUES(?, ?, ?, ?)
            "#,
        )
        .bind(ulid.to_string())
        .bind(dp.category)
        .bind(dp.material)
        .bind(dp.name)
        .execute(pool)
        .await?;

        Ok(())
    }
}
