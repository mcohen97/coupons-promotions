mod api_error;
mod evaluation_controller;
mod health_controller;
mod promotions_controller;
mod app_key_controller;
mod model_in;

pub use api_error::ApiError;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use evaluation_controller::EvaluationController;
use health_controller::HealthController;
use std::io;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use crate::models;
use crate::server::promotions_controller::PromotionsController;
use crate::messages::{MessageSender};
use std::io::ErrorKind;
use crate::server::app_key_controller::AppKeyController;

pub type ApiResult<T> = Result<T, ApiError>;

pub struct Server {
    config: ServerConfig
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Server { config }
    }

    pub fn start(&self) -> io::Result<()> {
        actix::System::new("sys");
        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(self.generate_database_url());
        let pool: models::Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let f = self.config.logger_format.to_string();
        let message_sender = MessageSender::new(&self.config.rabbit_url)
            .map_err(|e| std::io::Error::new(ErrorKind::ConnectionAborted, e))?;

        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .data(message_sender.clone())
                .wrap(middleware::Logger::new(&f))
                .data(
                    web::JsonConfig::default()
                        .content_type(|mime| mime == mime::APPLICATION_JSON)
                        .error_handler(|err, _req| {
                            error::InternalError::from_response(
                                "",
                                HttpResponse::BadRequest().json(ApiError::from(format!("Wrong format: {}", err))),
                            ).into()
                        }),
                )
                .service(web::resource("/health").route(web::get().to_async(HealthController::get)))
                .service(
                    web::resource("/evaluations/{id}")
                        .route(web::post().to(EvaluationController::post)),
                )
                .service(
                    web::scope("/promotions")
                        .service(
                            web::resource("")
                                .route(web::post().to(PromotionsController::post))
                                .route(web::get().to(PromotionsController::get_all))
                        )
                        .service(
                            web::resource("{id}")
                                .route(web::put().to(PromotionsController::put))
                                .route(web::get().to(PromotionsController::get))
                                .route(web::delete().to(PromotionsController::delete))
                        )
                )
                .service(
                    web::resource("app_key").route(web::post().to(AppKeyController::post))
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
    pub db_password: String,
    pub rabbit_url: String,
    pub logger_format: String,
}
