//use thiserror::Error;
use derive_more::{Display, Error};
use ntex::http;
use ntex::web;

#[derive(Debug, Display, Error)]
pub enum MenuError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl web::error::WebResponseError for MenuError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            MenuError::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}