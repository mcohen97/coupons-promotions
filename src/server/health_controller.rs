use actix_web::HttpResponse;
use crate::server::api_error::APIError;

pub struct HealthController;

impl HealthController {
    pub fn get() -> HttpResponse {
        let message = "Healthy".into();
        HttpResponse::Ok().json(HealthOut { message })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HealthOut {
    message: String
}

impl From<&'static str> for HealthOut {
    fn from(message: &'static str) -> Self {
        HealthOut { message: message.into() }
    }
}