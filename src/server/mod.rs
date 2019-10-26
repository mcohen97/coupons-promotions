mod api_error;
mod health_controller;
mod evaluation_controller;

pub use api_error::Result;
use health_controller::HealthController;
use evaluation_controller::EvaluationController;
use actix_web::{HttpServer, App, middleware, web, Responder, HttpResponse, HttpRequest};
use std::io;

pub struct Server {
    address: String
}

impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        let address = format!("{}:{}", host, port);

        Server {address}
    }

    pub fn start(&self) -> io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(
                    web::resource("/health")
                        .route(web::get().to(HealthController::get))
                )
                .service(web::resource("/evaluations/{id}")
                        .route(web::post().to(EvaluationController::post)))
        })
            .bind(self.address.clone())?
            .run()
    }
}