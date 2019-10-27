use actix_web::HttpResponse;
use iata_types::CityCodeParseError;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    pub message: String,
}

impl Into<HttpResponse> for APIError {
    fn into(self) -> HttpResponse {
        HttpResponse::BadRequest().json(self)
    }
}

impl From<String> for APIError {
    fn from(message: String) -> Self {
        APIError { message }
    }
}

impl From<&'static str> for APIError {
    fn from(message: &'static str) -> Self {
        APIError {
            message: message.into(),
        }
    }
}

impl From<CityCodeParseError> for APIError {
    fn from(_error: CityCodeParseError) -> APIError {
        "Invalid city code (must be AIATA)".into()
    }
}
