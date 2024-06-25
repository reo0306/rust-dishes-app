#[derive(Debug)]
pub struct FetchCategory {
    pub category: String,
}

#[derive(Debug)]
pub struct FetchCategoryDishes {
    pub category: String,
    pub material: String,
}

#[derive(Debug)]
pub struct Dishes {
    pub category: String,
    pub material: String,
    pub name: String,
}
