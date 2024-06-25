use ntex::web;

use crate::driver::routes::menu;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/menu").service((
                    web::scope("/{category}").service((
                        web::resource("")
                            .route(web::get().to(menu::category)),
                        web::scope("/{material}").service((
                            web::resource("")
                                .route(web::get().to(menu::category_dishes)),
                            web::scope("/{name}").service((
                                web::resource("")
                                    .route(web::get().to(menu::find_dishes))
                                    .route(web::post().to(menu::create_dishes)),
                            )),
                        )),
                    )),
                )),
    );
}