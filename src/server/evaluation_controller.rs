use actix_web::HttpRequest;
use crate::server::ApiResult;
use crate::services::*;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use http::header;

pub struct EvaluationController;

impl EvaluationController {
    pub fn post(path: web::Path<String>, data: Json<EvaluationIn>, req: HttpRequest) -> ApiResult<HttpResponse> {
        let eval_service = EvaluationService::new();
        let demo_service = DemographyService::new();
        let data = data.into_inner();
        let code = path.into_inner();
        let attributes = data.attributes.unwrap_or(HashMap::new());
        let demography = data.demography;
        let app_key = Self::get_authorization(&req);

        let result = eval_service.evaluate(code, attributes, app_key)?;
        demo_service.publish(DemographyIn {
            country: demography.country,
            city: demography.city,
            birth_date: demography.birth_date,
        })?;

        Ok(HttpResponse::Ok().json(result))
    }

    fn get_authorization(req: &HttpRequest) -> &str {
        req.headers()
        .get(header::AUTHORIZATION)
        .map(header::HeaderValue::to_str)
        .expect("Header contains non-ASCII characters")
        .unwrap_or("")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationIn {
    pub attributes: Option<HashMap<String, f64>>,
    pub demography: DemographyIn,
}
