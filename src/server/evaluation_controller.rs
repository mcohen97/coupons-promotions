use actix_web::{HttpResponse, web};
use crate::server::api_error::APIError;
use actix_web::web::Json;

pub struct EvaluationController;

impl EvaluationController {
    pub fn post(path: web::Path<u32>, data: Json<EvaluationIn>) -> HttpResponse {
        let id = path.into_inner();

        println!("{}",id);

        HttpResponse::Ok().finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationIn {

}