pub type Result<T> = actix_web::Result<T, APIError>;


#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    pub message: String
}

impl From<String> for APIError {
    fn from(message: String) -> Self {
        APIError { message }
    }
}

impl From<&'static str> for APIError {
    fn from(message: &'static str) -> Self {
        APIError { message: message.into() }
    }
}