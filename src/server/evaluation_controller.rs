#![allow(dead_code, unused_variables, unused_imports)]

use actix_web::HttpRequest;
use crate::server::{ApiResult, ApiError};
use crate::services::*;
use actix_web::web::{Json, Data};
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use http::header;
use crate::models::{Promotion, PromotionRepo, Pool, Connection, Demographics};
use diesel::RunQueryDsl;
use std::error::Error;
use chrono::Utc;
use std::time::SystemTime;


pub struct EvaluationController;

impl EvaluationController {
    pub fn post(id: web::Path<i32>, data: Json<EvaluationIn>, _pool: web::Data<Pool>, req: HttpRequest) -> ApiResult<HttpResponse> {
        let start = SystemTime::now();
        let (eval_service, demo_service) = Self::setup_services(_pool);
        let EvaluationIn { required, attributes, demographic_data } = data.into_inner();
        let id = id.into_inner();

        let eval_result = eval_service.evaluate_promotion(id, required, attributes)?;

        let demo = match demographic_data {
            Some(DemographyIn{country, city, birth_date}) => Demographics::new(&country,&city,&birth_date).ok(),
            None => None
        };

        let response_time = start.elapsed()?;
        let res = match eval_result {
            EvaluationResult::Applies {organization_id, total_discount} => EvaluationOut {
                promotion_id: id,
                organization_id,
                demographic_data: demo,
                evaluation_info: EvaluationInfo {
                    applicable: true,
                    total_discounted: Some(total_discount),
                    response_time: response_time.as_secs_f64(),
                }
            },
            EvaluationResult::DoesntApply {organization_id} => EvaluationOut {
                promotion_id: id,
                organization_id,
                demographic_data: None,
                evaluation_info: EvaluationInfo {
                    applicable: false,
                    total_discounted: None,
                    response_time: response_time.as_secs_f64(),
                }
            }
        };
        Ok(HttpResponse::Ok().json(res))
    }


    fn setup_services(pool: Data<Pool>) -> (EvaluationService, DemographyService) {
        let con = Box::new(pool.get().unwrap());
        let repo = Box::new(PromotionRepo::new(con));
        let eval_service = EvaluationService::new(repo);
        let demo_service = DemographyService::new();

        return (eval_service, demo_service);
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
    pub demographic_data: Option<DemographyIn>,
    #[serde(flatten)]
    pub required: RequiredAttribute,
}

#[derive(Serialize)]
pub struct EvaluationOut<'a> {
    pub promotion_id: i32,
    pub organization_id: i32,
    pub evaluation_info: EvaluationInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic_data: Option<Demographics<'a>>,
}

#[derive(Serialize)]
pub struct EvaluationInfo {
    pub applicable: bool,
    pub response_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discounted: Option<f64>,
}
