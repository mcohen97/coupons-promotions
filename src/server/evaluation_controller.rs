use actix_web::HttpRequest;
use crate::server::ApiResult;
use crate::services::*;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use http::header;
use crate::models::{Promotion, PromotionRepository, Pool, PromotionReturn};
use diesel::RunQueryDsl;
use std::error::Error;
use std::rc::Rc;


pub struct EvaluationController;

impl EvaluationController {
    pub fn post(path: web::Path<i32>, data: Json<EvaluationIn>, _pool: web::Data<Pool>, req: HttpRequest) -> ApiResult<HttpResponse> {
        let con = _pool.get().unwrap();
        let repo = PromotionRepository::new(Rc::new(con));
        let eval_service = EvaluationServices::new(repo);
        let demo_service = DemographyServices::new();
        let EvaluationIn { required, attributes, demography} = data.into_inner();

        let eval_result = eval_service.evaluate_promotion(path.into_inner(), required, attributes)?;


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
            EvaluationResult::Applies(ret) => unreachable!(),
            EvaluationResult::DoesntApply => EvaluationOut {is_valid: false, return_type: None, return_val: None}
        }
    }
}