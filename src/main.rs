use ntex::web;

use dishes_app::driver::config;
use dishes_app::adapter::persistence::database::Db;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let db = Db::new().await;

    web::HttpServer::new(move || {
        web::App::new()
        .state(db.clone())
        .configure(config::config_app)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
