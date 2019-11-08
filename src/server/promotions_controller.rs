use futures::Future;
use actix_web::{web, HttpResponse};
use crate::server::{ApiError, ApiResult};
use actix_web::web::{Json};
use crate::models::{NewPromotion, Pool, Promotion, PromotionType};

pub struct PromotionsController;

impl PromotionsController {
    pub fn post(data: Json<PromotionIn>,  pool: web::Data<Pool>) -> ApiResult<HttpResponse> {
        let data = data.into_inner();

        Ok(HttpResponse::Ok().finish())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromotionIn {
    pub code: String,
    pub name: String,
    pub return_type: String,
    pub return_value: f64,
    pub promotion_type: PromotionType,
    pub organization_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReturnTypesIn {
    Percentage,
    Fixed
}