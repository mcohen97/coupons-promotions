mod api_error;
mod evaluation_controller;
mod health_controller;


use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
pub use api_error::{ApiError};
use evaluation_controller::{EvaluationController};
use health_controller::HealthController;
use std::io;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use crate::models;

pub type ApiResult<T> = Result<T, ApiError>;

pub struct Server {
    config: ServerConfig
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Server { config }
    }

    pub fn start(&self) -> io::Result<()> {
        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(self.generate_database_url());
        let pool: models::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");


        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .data(
                    web::JsonConfig::default()
                        .content_type(|mime| mime == mime::APPLICATION_JSON)
                        .error_handler(|err, _req| {
                            error::InternalError::from_response(
                                err,
                                HttpResponse::BadRequest().json(ApiError::from("Wrong format")),
                            )
                            .into()
                        }),
                )
                .service(web::resource("/health").route(web::get().to_async(HealthController::get)))
                .service(
                    web::resource("/evaluations/{code}")
                        .route(web::post().to(EvaluationController::post)),
                )
        })
        .bind(format!("{}:{}", &self.config.domain, &self.config.port))?
        .run()
    }

    fn generate_database_url(&self) -> String {
        format!("postgres://{}:{}@{}/{}", &self.config.db_user, &self.config.db_password, &self.config.db_host, &self.config.db_name)
    }
}

pub struct ServerConfig {
    pub domain: String,
    pub port: u16,
    pub db_host: String,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String
}
