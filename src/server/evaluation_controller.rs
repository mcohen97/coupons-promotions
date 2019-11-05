#![allow(dead_code, unused_variables, unused_imports)]
use actix_web::HttpRequest;
use crate::server::ApiResult;
use crate::services::*;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use http::header;
use crate::models::*;
use diesel::RunQueryDsl;
use std::error::Error;


pub struct EvaluationController;

impl EvaluationController {
    pub fn post(path: web::Path<String>, data: Json<EvaluationIn>, _pool: web::Data<Pool>, req: HttpRequest) -> ApiResult<HttpResponse> {
        let con = _pool.get().unwrap();
        let eval_service = EvaluationService::new(con);
        let demo_service = DemographyService::new();

        let data = data.into_inner();
        let c_code: String = path.into_inner();
        let attributes = data.attributes.unwrap_or(HashMap::new());
        let demography = data.demography;
        let _app_key = Self::get_authorization(&req);

        //let result = eval_service.evaluate(c_code, attributes, &app_key)?;
        if let Some(demography) = demography {
            demo_service.publish(DemographyIn {
                country: demography.country,
                city: demography.city,
                birth_date: demography.birth_date,
            })?;
        }
/*
        let con = pool.get().unwrap();
        let post = Promotion::default();
        let inserted: Promotion = diesel::insert_into(promotions)
            .values(&post)
            .get_result(&con)?;
*/
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

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationIn {
    pub attributes: Option<HashMap<String, f64>>,
    pub demography: Option<DemographyIn>,
}
