mod api_error;
mod evaluation_controller;
mod health_controller;

use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
pub use api_error::{APIError};
use evaluation_controller::{EvaluationController};
use health_controller::HealthController;
use std::io;

pub type ApiResult<T> = Result<T, APIError>;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        let address = format!("{}:{}", host, port);
        Server { address }
    }

    pub fn start(&self) -> io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .data(
                    web::JsonConfig::default()
                        .content_type(|mime| mime == mime::APPLICATION_JSON)
                        .error_handler(|err, _req| {
                            error::InternalError::from_response(
                                err,
                                HttpResponse::BadRequest().json(APIError::from("Wrong format")),
                            )
                            .into()
                        }),
                )
                .service(web::resource("/health").route(web::get().to(HealthController::get)))
                .service(
                    web::resource("/evaluations/{code}")
                        .route(web::post().to(EvaluationController::post)),
                )
        })
        .bind(self.address.clone())?
        .run()
    }
}
