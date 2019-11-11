use futures::Future;
use actix_web::{web, HttpResponse};
use crate::server::{ApiError, ApiResult};
use actix_web::web::Json;
use crate::models::{NewPromotion, Pool, Promotion, PromotionType, PromotionReturn, PromotionRepo, PromotionExpression};
use std::rc::Rc;

pub struct PromotionsController;

impl PromotionsController {
    pub fn post(data: Json<PromotionIn>, pool: web::Data<Pool>) -> ApiResult<HttpResponse> {
        let repo = Self::setup_repo(pool);
        let new_promotion = Self::build_new_promotion(data);
        Self::validate_code(&new_promotion.code)?;
        let created = repo.create(&new_promotion)?;

        Ok(HttpResponse::Created().json(created))
    }

    pub fn put(id: web::Path<i32>, data: Json<PromotionIn>, pool: web::Data<Pool>) -> ApiResult<HttpResponse> {
        let repo = Self::setup_repo(pool);
        let id = id.into_inner();

        let mut promotion = repo.find(id)?;
        let PromotionIn { name, code: code, return_type, return_value, promotion_type, organization_id } = data.into_inner();
        promotion = Promotion { name, code: code.to_lowercase(), return_type: return_type.to_string(), return_value, type_: promotion_type.to_string(), organization_id, ..promotion };
        Self::validate_code(&promotion.code)?;
        repo.update(&promotion)?;

        Ok(HttpResponse::Ok().json(promotion))
    }

    pub fn delete(id: web::Path<i32>, pool: web::Data<Pool>) -> ApiResult<HttpResponse> {
        let repo = Self::setup_repo(pool);
        let id = id.into_inner();
        repo.delete(id)?;

        Ok(HttpResponse::Ok().finish())
    }

    pub fn get(id: web::Path<i32>, pool: web::Data<Pool>) -> ApiResult<HttpResponse> {
        let repo = Self::setup_repo(pool);
        let id = id.into_inner();
        let promo = repo.find(id)?;

        Ok(HttpResponse::Ok().json(&promo))
    }

    pub fn get_all(pool: web::Data<Pool>) -> ApiResult<HttpResponse> {
        let repo = Self::setup_repo(pool);
        let promos = repo.get()?;

        Ok(HttpResponse::Ok().json(&promos))
    }

    fn validate_code(code: &str) -> ApiResult<()> {
        PromotionExpression::parse(code)?;
        Ok(())
    }

    fn build_new_promotion(data: Json<PromotionIn>) -> NewPromotion {
        let PromotionIn { name, code, return_type, return_value, promotion_type, organization_id } = data.into_inner();
        let ret = return_type.get_return(return_value);
        NewPromotion::new(
            name,
            code.to_lowercase(),
            true,
            ret,
            promotion_type,
            organization_id,
        )
    }

    fn setup_repo(pool: web::Data<Pool>) -> PromotionRepo {
        let con = Rc::new(pool.get().unwrap());
        PromotionRepo::new(con)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromotionIn {
    pub code: String,
    pub name: String,
    pub return_type: ReturnTypesIn,
    pub return_value: f64,
    pub promotion_type: PromotionType,
    pub organization_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReturnTypesIn {
    Percentage,
    Fixed,
}

impl ReturnTypesIn {
    pub fn get_return(&self, value: f64) -> PromotionReturn {
        match self {
            ReturnTypesIn::Percentage => PromotionReturn::Percentage(value),
            ReturnTypesIn::Fixed => PromotionReturn::Fixed(value)
        }
    }
}

impl ReturnTypesIn {
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}