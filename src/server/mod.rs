mod api_error;
mod evaluation_controller;
mod health_controller;
mod promotions_controller;
mod coupons_controller;
mod app_key_controller;
mod model_in;
mod service_factory;

pub use api_error::ApiError;
pub use model_in::*;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use evaluation_controller::EvaluationController;
use health_controller::HealthController;
use std::io;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use crate::models;
use crate::server::promotions_controller::PromotionsController;
use crate::messages::{MessageSender, MessageListener};
use std::io::ErrorKind;
use crate::server::app_key_controller::AppKeyController;
use crate::models::{OrganizationRepository};
use std::rc::Rc;

pub type ApiResult<T> = Result<T, ApiError>;

pub use service_factory::ServiceFactory;
use std::sync::Arc;
use crate::server::coupons_controller::CouponsController;

pub struct Server {
    config: ServerConfig
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Server { config }
    }

    pub fn start(&self) -> io::Result<()> {
        let _ = actix::System::new("sys");
        let logger_format = self.config.logger_format.to_string();

        let pool = self.get_pool();
        let (message_sender, _message_listener) = self.start_message_handlers(pool.clone())?;
        let arc_sender = Arc::new(message_sender.clone());

        HttpServer::new(move || {
            App::new()
                .data(message_sender.clone())
                .data(ServiceFactory::new(Self::get_pool_s(), arc_sender.clone()))
                .wrap(middleware::Logger::new(&logger_format))
                .data(Self::error_handling())
                .service(web::resource("/health").route(web::get().to_async(HealthController::get)))
                .service(
                    web::scope("/promotions")
                        .service(
                            web::resource("")
                                .route(web::post().to(PromotionsController::post))
                                .route(web::get().to(PromotionsController::get_all))
                        )
                        .service(
                            web::resource("{id}/evaluations")
                                .route(web::post().to(EvaluationController::post)),
                        )
                        .service(
                            web::resource("{id}")
                                .route(web::put().to(PromotionsController::put))
                                .route(web::get().to(PromotionsController::get))
                                .route(web::delete().to(PromotionsController::delete))
                        )
                        .service(
                            web::resource("{id}/coupons")
                                .route(web::post().to(CouponsController::post))
                                .route(web::get().to(CouponsController::get))
                        )
                )
                .service(
                    web::resource("app_key").route(web::post().to(AppKeyController::post))
                )
        })
            .bind(format!("{}:{}", &self.config.domain, &self.config.port))?
            .run()
    }

    fn error_handling() -> web::JsonConfig {
        web::JsonConfig::default()
            .content_type(|mime| mime == mime::APPLICATION_JSON)
            .error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest().json(ApiError::from(format!("Wrong format: {}", err))),
                ).into()
            })
    }

    fn get_pool(&self) -> models::Pool {
        let manager = ConnectionManager::<PgConnection>::new(self.generate_database_url());
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    fn get_pool_s() -> models::Pool {
        let manager = ConnectionManager::<PgConnection>::new("postgres://coupons:coupons@localhost/eval-dev");
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }


    fn start_message_handlers(&self, pool: models::Pool) -> io::Result<(MessageSender, MessageListener)> {
        let message_sender = MessageSender::new(&self.config.rabbit_url)
            .map_err(|e| std::io::Error::new(ErrorKind::ConnectionAborted, e))?;
        let message_listener = MessageListener::new(&self.config.rabbit_url, OrganizationRepository::new(Rc::new(pool.get().unwrap())))
            .map_err(|e| std::io::Error::new(ErrorKind::ConnectionAborted, e))?;
        message_listener.run();

        Ok((message_sender, message_listener))
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
