mod api_error;
mod evaluation_controller;
mod health_controller;
mod promotions_controller;
mod coupons_controller;
mod app_key_controller;
mod model_in;
mod service_factory;
mod authenticater;

pub use api_error::ApiError;
pub use model_in::*;
pub use service_factory::ServiceFactory;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use evaluation_controller::EvaluationController;
use health_controller::HealthController;
use std::io;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::io::ErrorKind;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use crate::models;
use crate::server::promotions_controller::PromotionsController;
use crate::messages::{MessageSender, MessageListener, RabbitSender, Message};
use crate::server::app_key_controller::AppKeyController;
use crate::models::OrganizationRepository;
use crate::server::coupons_controller::CouponsController;

pub type ApiResult<T> = Result<T, ApiError>;

const ADMIN_PERM: &str = "ADMIN";
const HEADER: &str = "authentication";
lazy_static! {
    static ref SECRET: String = std::env::var("SECRET").expect("Missing SECRET");
}

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

        HttpServer::new(move || {
            App::new()
                .data(ServiceFactory::new(pool.clone(), message_sender.clone()))
                .wrap(middleware::Logger::new(&logger_format))
                .data(Self::error_handling())
                .service(
                    web::scope("/v1")
                        .service(web::resource("/health").route(web::get().to_async(HealthController::get)))
                        .service(
                            web::scope("/promotions")
                                .service(
                                    web::resource("")
                                        .route(web::post().to(PromotionsController::post))
                                        .route(web::get().to(PromotionsController::get_all))
                                )
                                .service(
                                    web::resource("{id}/evaluate")
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
                            web::scope("app_keys")
                                .service(
                                    web::resource("")
                                        .route(web::post().to(AppKeyController::post))
                                        .route(web::get().to(AppKeyController::get_all))
                                )
                                .service(
                                    web::resource("{token}")
                                        .route(web::get().to(AppKeyController::get))
                                )
                        )
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
            .max_size(std::env::var("MAX_CONNECTIONS").expect("MAX_CONNECTIONS missing").parse().expect("Error parsing max connections"))
            .build(manager)
            .expect("Failed to create pool.")
    }

    fn start_message_handlers(&self, pool: models::Pool) -> io::Result<(MessageSender, MessageListener)> {
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
        let mut rabbbit = RabbitSender::new(&self.config.rabbit_url, rx)
            .map_err(|e| std::io::Error::new(ErrorKind::ConnectionAborted, e))?;
        std::thread::spawn(move || {
            rabbbit.start();
        });
        let message_sender = MessageSender::new(tx.clone());
        let message_listener = MessageListener::new(&self.config.rabbit_url, OrganizationRepository::new(Rc::new(pool.get().unwrap())))
            .map_err(|e| std::io::Error::new(ErrorKind::ConnectionAborted, e))?;
        message_listener.run();

        Ok((message_sender, message_listener))
    }

    fn generate_database_url(&self) -> String {
        std::env::var("DATABASE_URL").expect("DATABASE_URL missing")
    }
}

pub struct ServerConfig {
    pub domain: String,
    pub port: u16,
    pub rabbit_url: String,
    pub logger_format: String,
}
