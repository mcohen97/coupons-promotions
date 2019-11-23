use crate::server::{ApiResult, ServiceFactory};
use crate::services::*;
use actix_web::web::{Json, Data};
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use crate::models::PromotionReturn;
use std::time::Duration;
use crate::messages::{EvaluationInfo, DemographyData, Message};
use crate::messages;
use crate::server::authenticater::Authorization;

lazy_static! {
    static ref POST_PERMS: Vec<&'static str> = vec!["ADMIN"];
}

pub struct EvaluationController;
impl EvaluationController {
    pub fn post(path: web::Path<i32>, data: Json<EvaluationIn>, fact: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &POST_PERMS)?;
        let start = std::time::SystemTime::now();
        let id = path.into_inner();
        let Services { evaluation, demographic, message_sender, .. } = fact.as_services()?;
        let EvaluationIn { attributes, demography, specific_data , token} = data.into_inner();

        let eval_result = evaluation.evaluate_promotion(id, specific_data, attributes, token, org)?;
        let response_time = start.elapsed().unwrap();
        let (demo_response, demo_data) = demographic.build_demographics_if_valid(demography);
        message_sender.send(Message::PromotionEvaluated(eval_result.to_message(id, response_time, demo_data)));

        Ok(HttpResponse::Ok().json(eval_result.to_out(demo_response.to_string())))
    }
}

impl EvaluationResultDto {
    pub fn to_message(&self, promotion_id: i32, response_time: Duration, demographic_data: Option<DemographyData>) -> messages::EvaluationResult {
        match self {
            EvaluationResultDto::Applies { organization_id, total_discount, .. } => messages::EvaluationResult {
                organization_id: organization_id.to_string(),
                promotion_id,
                demographic_data,
                result: EvaluationInfo {
                    total_discounted: Some(*total_discount),
                    applicable: true,
                    response_time: response_time.as_millis(),
                },
            },
            EvaluationResultDto::DoesntApply { organization_id } => messages::EvaluationResult {
                organization_id: organization_id.to_string(),
                promotion_id,
                demographic_data,
                result: EvaluationInfo {
                    total_discounted: None,
                    applicable: false,
                    response_time: response_time.as_millis(),
                },
            }
        }
    }

    fn to_out(&self, demography_response: String) -> EvaluationOut {
        match self {
            EvaluationResultDto::Applies { total_discount, return_type, .. } => EvaluationOut {
                is_valid: true,
                return_type: Some(return_type.to_string()),
                return_val: Some(*total_discount),
                demography_response,
            },
            EvaluationResultDto::DoesntApply { .. } => EvaluationOut {
                is_valid: true,
                return_val: None,
                return_type: None,
                demography_response,
            }
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct EvaluationIn {
    pub attributes: HashMap<String, f64>,
    pub demography: Option<DemographyIn>,
    #[serde(flatten)]
    pub specific_data: EvaluationSpecificDto,
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationOut {
    pub is_valid: bool,
    pub return_type: Option<String>,
    pub return_val: Option<f64>,
    pub demography_response: String,
}

#[derive(Serialize, Deserialize)]
pub enum EvaluationResult {
    Applies(PromotionReturn),
    DoesntApply,
}