use sqlx::FromRow;
use chrono::{DateTime, Local};
use serde::Serialize;

use crate::domain::model::dishes::{Dishes, FetchCategory, FetchCategoryDishes};

#[derive(FromRow, Serialize)]
pub struct DishesTable {
    pub id: String,
    pub category: String,
    pub material: String,
    pub name: String,
    pub create_date: DateTime<Local>,
    pub update_date: DateTime<Local>,
}

pub struct FetchCategoryParam {
    pub category: String,
}

pub struct FetchCategoryDishesParam {
    pub category: String,
    pub material: String,
}

pub struct DishesParam {
    pub category: String,
    pub material: String,
    pub name: String,
}

impl From<FetchCategory> for FetchCategoryParam {
    fn from(fc: FetchCategory) -> Self {
        FetchCategoryParam {
            category: fc.category,
        }
    }
}

impl From<FetchCategoryDishes> for FetchCategoryDishesParam {
    fn from(fcd: FetchCategoryDishes) -> Self {
        FetchCategoryDishesParam {
            category: fcd.category,
            material: fcd.material,
        }
    }
}

impl From<Dishes> for DishesParam {
    fn from(d: Dishes) -> Self {
        DishesParam {
            category: d.category,
            material: d.material,
            name: d.name,
        }
    }
}