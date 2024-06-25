use dotenv::dotenv;
use sqlx::{MySql, Pool, MySqlPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Pool<MySql>>);

impl Db {
    pub async fn new() -> Self {
        dotenv().ok();

        let user = dotenv::var("USER").unwrap();
        let password = dotenv::var("PASS").unwrap();
        let host = dotenv::var("HOST").unwrap();
        let port = dotenv::var("PORT").unwrap();
        let database = dotenv::var("DATABASE").unwrap();

        let database_url: String = format!("mysql://{user}:{password}@{host}:{port}/{database}");

        let pool = MySqlPool::connect(&database_url).await;

        pool.map_or_else(
            |_| panic!("not connect db"),
            |pool| Self(Arc::new(pool))
        ) 
    }
}

#[cfg(test)]
mod test_db {
    use super::*;
    use rstest::{fixture, rstest};
    use claim::*;

    #[fixture]
    async fn db() -> Db {
        Db::new().await
    }

    #[rstest]
    #[ntex::test]
    async fn should_connect_database(#[future(awt)] db: Db ) {
        let db = db.0.clone();
        assert_ok!(db.acquire().await);
    }
}