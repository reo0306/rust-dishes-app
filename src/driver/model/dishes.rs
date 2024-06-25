use serde::Deserialize;

use crate::domain::model::dishes::{FetchCategory, FetchCategoryDishes, Dishes};

#[derive(Debug, Deserialize)]
pub struct Category{
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct CategoryDishes {
    pub category: String,
    pub material: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestDishes {
    pub category: String,
    pub material: String,
    pub name: String,
}

impl From<Category> for FetchCategory {
    fn from(c: Category) -> Self {
        FetchCategory {
            category: c.category,
        }
    }
}

impl From<CategoryDishes> for FetchCategoryDishes {
    fn from(cd: CategoryDishes) -> Self {
        FetchCategoryDishes {
            category: cd.category,
            material: cd.material,
        }
    }
}

impl From<RequestDishes> for Dishes {
    fn from(rd: RequestDishes) -> Self {
        Dishes {
            category: rd.category,
            material: rd.material,
            name: rd.name,
        }
    }
}