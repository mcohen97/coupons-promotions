use actix_web::HttpResponse;
use actix_web::ResponseError;
use core::fmt::Display;
use iata_types::CityCodeParseError;
use std::error::Error;
use evalexpr::EvalexprError;
use std::borrow::Cow;

type Message = Cow<'static, str>;

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    BadRequest(Message),
    InternalError(Message),
}

impl Into<HttpResponse> for ApiError {
    fn into(self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ApiError::InternalError(msg) => HttpResponse::InternalServerError().json(msg)
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ApiError::InternalError(msg) => HttpResponse::InternalServerError().json(msg)
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            ApiError::BadRequest(msg) => f.write_str(msg),
            ApiError::InternalError(msg) => f.write_str(msg)
        }
    }
}

impl From<String> for ApiError {
    fn from(message: String) -> Self {
        ApiError::BadRequest(message.into())
    }
}

impl From<&'static str> for ApiError {
    fn from(message: &'static str) -> Self {
        ApiError::BadRequest(message.into())
    }
}

impl From<CityCodeParseError> for ApiError {
    fn from(_error: CityCodeParseError) -> ApiError {
        "Invalid city code (must be AIATA)".into()
    }
}

impl From<evalexpr::EvalexprError> for ApiError {
    fn from(err: EvalexprError) -> Self {
        err.to_string().into()
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::NotFound => "Not found".into(),
            _ => ApiError::InternalError(Cow::from("Data access error"))
        }
    }
}