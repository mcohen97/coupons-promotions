use crate::models::DemographicData;
use crate::server::api_error::APIError;
use crate::server::ApiResult;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use std::collections::HashMap;

pub struct EvaluationController;

impl EvaluationController {
    pub fn post(path: web::Path<String>, data: Json<EvaluationIn>) -> HttpResponse {
        let code = path.into_inner();

        if let Err(err) = Self::send_demographics(&data) {
            return err.into()
        }
        
        

        HttpResponse::Ok().finish()
    }

    fn send_demographics(data: &Json<EvaluationIn>) -> ApiResult<()> {
        if let (Some(country), Some(city), Some(birth_date)) =
            (&data.country, &data.city, &data.birth_date)
        {
            let demo_data = DemographicData::new(country, city, birth_date)?;
            demo_data.publish_data()?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationIn {
    attributes: Option<HashMap<String, f64>>,
    city: Option<String>,
    country: Option<String>,
    birth_date: Option<String>,
}
