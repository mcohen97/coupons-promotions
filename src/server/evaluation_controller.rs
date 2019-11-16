use actix_web::HttpRequest;
use crate::server::{ApiResult, ServiceFactory};
use crate::services::*;
use actix_web::web::{Json, Data};
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use http::header;
use crate::models::{Pool, PromotionReturn};
use std::error::Error;


pub struct EvaluationController;

impl EvaluationController {
    pub fn post(path: web::Path<i32>, data: Json<EvaluationIn>, fact: Data<ServiceFactory>) -> ApiResult<HttpResponse> {
        let eval_service = fact.as_services()?.evaluation;
        let EvaluationIn { required, attributes, demography: _} = data.into_inner();

        let _eval_result = eval_service.evaluate_promotion(path.into_inner(), required, attributes)?;


        Ok(HttpResponse::Ok().finish())
    }

    fn get_authorization(req: &HttpRequest) -> String {
        req.headers()
            .get(header::AUTHORIZATION)
            .map(header::HeaderValue::to_str)
            .map(|r| match r {
                Ok(value) => value.to_string(),
                Err(err) => err.description().to_string()
            })
            .unwrap_or("".into())
    }
}

#[derive(Serialize, Deserialize)]
pub struct EvaluationIn {
    pub attributes: HashMap<String, f64>,
    pub demography: Option<DemographyIn>,
    pub required: RequiredAttribute
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationOut {
    pub is_valid: bool,
    pub return_type: Option<String>,
    pub return_val: Option<f64>
}

#[derive(Serialize, Deserialize)]
pub enum EvaluationResult {
    Applies(PromotionReturn),
    DoesntApply,
}

impl From<EvaluationResult> for EvaluationOut {
    fn from(res: EvaluationResult) -> Self {
        match res {
            EvaluationResult::Applies(_ret) => unreachable!(),
            EvaluationResult::DoesntApply => EvaluationOut {is_valid: false, return_type: None, return_val: None}
        }
    }
}