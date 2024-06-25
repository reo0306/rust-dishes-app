use ntex::web;

use crate::domain::model::dishes::{FetchCategory, FetchCategoryDishes, Dishes};
use crate::driver::{
    error::MenuError,
    model::dishes::{Category, CategoryDishes, RequestDishes}
};
use crate::adapter::{
    persistence::database::Db,
    repository::dishes::DishesRepository,
};

pub async fn category(
    db: web::types::State<Db>,
    path: web::types::Path<Category>
) -> Result<impl web::Responder, MenuError> {
    let fc: FetchCategory = path.into_inner().into();

    let categories = DishesRepository::fetch_categories(&db.0, fc).await;
    categories
    .map(|r| {
        web::HttpResponse::Ok().json(&r)
    })
    .map_err(|_| MenuError::InternalError)
}

pub async fn category_dishes(
    db: web::types::State<Db>,
    path: web::types::Path<CategoryDishes>
) -> Result<impl web::Responder, MenuError> {
    let fcd: FetchCategoryDishes = path.into_inner().into();

    let categories_dishes = DishesRepository::fetch_categories_dishes(&db.0, fcd).await;
    categories_dishes
    .map(|r| {
        web::HttpResponse::Ok().json(&r)
    })
    .map_err(|_| MenuError::InternalError)
}

pub async fn find_dishes(
    db: web::types::State<Db>,
    path: web::types::Path<RequestDishes>
) -> Result<impl web::Responder, MenuError> {
    let dishes: Dishes = path.into_inner().into();

    let find_dishes = DishesRepository::find_dishes(&db.0, dishes).await;
    find_dishes
    .map(|r| {
        web::HttpResponse::Ok().json(&r)
    })
    .map_err(|_| MenuError::InternalError)
}

pub async fn create_dishes(
    db: web::types::State<Db>,
    path: web::types::Path<RequestDishes>
) -> Result<impl web::Responder, MenuError> {
    let dishes: Dishes = path.into_inner().into();

    match DishesRepository::create_dishes(&db.0, dishes).await {
        Ok(r) => {
            Ok(web::HttpResponse::Ok().json(&r))
        },
        Err(_e) => {
            Err(MenuError::InternalError)
        }
    }
}

#[cfg(test)]
mod test_menu {
    use rstest::{fixture, rstest};
    use ntex::web;
    use ntex::web::{test, WebResponse, Error};
    use ntex::http::{StatusCode, Request};
    use ntex::{Pipeline, Service};

    use crate::driver::config;
    use crate::adapter::persistence::database::Db;

    #[fixture]
    async fn app() -> Pipeline<impl Service<Request, Response = WebResponse, Error = Error>> {
        let db = Db::new().await;

        test::init_service(
            web::App::new()
            .state(db.clone())
            .configure(config::config_app))
            .await
    }

    #[rstest]
    #[case("meat")]
    #[case("fish")]
    #[ntex::test]
    async fn test_category(
        #[future(awt)] app: Pipeline<impl Service<Request, Response = WebResponse, Error = Error>>,
        #[case] category: String,
    ) {
        let req = test::TestRequest::with_uri(format!("/menu/{category}").as_str()).to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[rstest]
    #[case("meat", "cow")]
    #[case("fish", "salmon")]
    #[ntex::test]
    async fn test_category_dishes(
        #[future(awt)] app: Pipeline<impl Service<Request, Response = WebResponse, Error = Error>>,
        #[case] category: String,
        #[case] material: String,
    ) {
        let req = test::TestRequest::with_uri(format!("/menu/{category}/{material}").as_str()).to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[rstest]
    #[case("meat", "cow", "Shabu-shabu")]
    #[case("fish", "salmon", "Salt-grilled")]
    #[ntex::test]
    async fn test_create_dishes(
        #[future(awt)] app: Pipeline<impl Service<Request, Response = WebResponse, Error = Error>>,
        #[case] category: String,
        #[case] material: String,
        #[case] name: String,
    ) {
        let uri = format!("/menu/{category}/{material}/{name}");
        let req = test::TestRequest::post().uri(uri.as_str()).to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[rstest]
    #[case("meat", "cow", "Shabu-shabu")]
    #[case("fish", "salmon", "Salt-grilled")]
    #[ntex::test]
    async fn test_fetch_dishes(
        #[future(awt)] app: Pipeline<impl Service<Request, Response = WebResponse, Error = Error>>,
        #[case] category: String,
        #[case] material: String,
        #[case] name: String,
    ) {
        let uri = format!("/menu/{category}/{material}/{name}");
        let req = test::TestRequest::with_uri(uri.as_str()).to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}