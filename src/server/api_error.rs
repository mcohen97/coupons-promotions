use actix_web::HttpResponse;
use actix_web::ResponseError;
use core::fmt::Display;
use iata_types::CityCodeParseError;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    pub message: String,
}

impl Into<HttpResponse> for APIError {
    fn into(self) -> HttpResponse {
        HttpResponse::BadRequest().json(self)
    }
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(self)
    }
}

impl Display for APIError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(&self.message)
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
