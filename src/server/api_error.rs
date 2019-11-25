use actix_web::HttpResponse;
use actix_web::ResponseError;
use iata_types::CityCodeParseError;
use evalexpr::EvalexprError;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Display;
use std::collections::HashMap;

type Message = Cow<'static, str>;

#[derive(Debug, Serialize, Deserialize)]
struct ErrorJson {
    error: Cow<'static, str>
}

#[derive(Debug, Serialize)]
pub struct InvalidFieldJson {
    #[serde(flatten)]
    m: HashMap<String, String>
}

impl InvalidFieldJson {
    pub fn new(field: &str, reason: &str) -> InvalidFieldJson {
        let mut m = HashMap::new();
        m.insert(field.to_string(), reason.to_string());
        InvalidFieldJson { m }
    }

    pub fn get_json(&self) -> String {
        let (field, reason) = self.m.iter().last().unwrap();
        format!("{{\n {} : {}\n }}", field, reason)
    }

    pub fn get_message(&self) -> String {
        let (field, reason) = self.m.iter().last().unwrap();
        format!("Field {} is invalid: {}", field, reason)
    }
}

impl ErrorJson {
    pub fn from_message<T>(msg: T) -> ErrorJson
        where T: Into<Message> {
        ErrorJson { error: msg.into() }
    }
}

#[derive(Debug, Serialize)]
pub enum ApiError {
    BadRequest(Message),
    BadRequestInvalidField(InvalidFieldJson),
    InternalError(Message),
    Unauthorized,
}

impl Into<HttpResponse> for ApiError {
    fn into(self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorJson::from_message(msg)),
            ApiError::BadRequestInvalidField(msg) => HttpResponse::BadRequest().json(msg),
            ApiError::InternalError(msg) => HttpResponse::InternalServerError().json(ErrorJson::from_message(msg)),
            ApiError::Unauthorized => HttpResponse::Unauthorized().json(ErrorJson::from_message("Unauthorized")),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().content_type("application/json").json(ErrorJson::from_message(msg.to_string())),
            ApiError::BadRequestInvalidField(msg) => HttpResponse::BadRequest().json(msg),
            ApiError::InternalError(msg) => HttpResponse::InternalServerError().json(ErrorJson::from_message(msg.to_string())),
            ApiError::Unauthorized => HttpResponse::Unauthorized().json(ErrorJson::from_message("Unauthorized")),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            ApiError::BadRequest(msg) => f.write_str(&serde_json::to_string_pretty(&ErrorJson::from_message(msg.to_string())).unwrap()),
            ApiError::BadRequestInvalidField(msg) => f.write_str(&serde_json::to_string_pretty(&msg).unwrap()),
            ApiError::InternalError(msg) => f.write_str(&serde_json::to_string_pretty(&ErrorJson::from_message(msg.to_string())).unwrap()),
            ApiError::Unauthorized => f.write_str(&serde_json::to_string_pretty(&ErrorJson::from_message("Unauthorized")).unwrap())
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
    fn from(e: EvalexprError) -> Self {
        ApiError::BadRequest(format!("Promotion code is invalid: {}", e).into())
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::NotFound => "Not found".into(),
            e => ApiError::InternalError(Cow::from(format!("Data access error: {}", e.description())))
        }
    }
}

impl From<std::time::SystemTimeError> for ApiError {
    fn from(e: std::time::SystemTimeError) -> Self {
        ApiError::InternalError(Cow::from(e.to_string()))
    }
}

impl From<lapin::Error> for ApiError {
    fn from(e: lapin::Error) -> Self {
        ApiError::InternalError(Cow::Owned(e.description().to_string()))
    }
}

impl From<r2d2::Error> for ApiError {
    fn from(e: r2d2::Error) -> Self {
        ApiError::InternalError(e.description().to_string().into())
    }
}

impl ApiError {
    pub fn get_message(&self) -> Cow<'static, str> {
        match self {
            ApiError::InternalError(m) => m.clone(),
            ApiError::BadRequest(m) => m.clone(),
            ApiError::BadRequestInvalidField(m) => m.get_message().into(),
            ApiError::Unauthorized => "Unauthorized".into()
        }
    }

    pub fn invalid_field(field: &str, reason: &str) -> ApiError {
        ApiError::BadRequestInvalidField(InvalidFieldJson::new(field, reason))
    }
}