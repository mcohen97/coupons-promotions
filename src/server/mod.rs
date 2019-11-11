mod api_error;
mod evaluation_controller;
mod health_controller;
mod promotions_controller;
mod model_in;

use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
pub use api_error::ApiError;
use evaluation_controller::EvaluationController;
use health_controller::HealthController;
use std::io;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use crate::models;
use crate::server::promotions_controller::PromotionsController;
use crate::services::MessageListener;
use actix::ContextFutureSpawner;

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

        let m = MessageListener::new("amqp://lyepjabq:DDt-OwA5B7XOCswfKgthGwA59yA1P73w@prawn.rmq.cloudamqp.com/lyepjabq");
        let f = m.start();
        actix::spawn(f);

        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .data(
                    web::JsonConfig::default()
                        .content_type(|mime| mime == mime::APPLICATION_JSON)
                        .error_handler(|err, _req| {
                            error::InternalError::from_response(
                                "",
                                HttpResponse::BadRequest().json(ApiError::from(format!("Wrong format: {}", err))),
                            )
                                .into()
                        }),
                )
                .service(web::resource("/health").route(web::get().to_async(HealthController::get)))
                .service(
                    web::resource("/evaluations/{code}")
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
}
