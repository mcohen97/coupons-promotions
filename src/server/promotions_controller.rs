use actix_web::{web, HttpResponse};
use crate::server::ApiResult;
use actix_web::web::Json;
use crate::models::{NewPromotion, Pool, Promotion, PromotionRepo, PromotionExpression};
use std::rc::Rc;
use crate::server::model_in::*;
use crate::messages::{MessageSender, Message};

pub struct PromotionsController;

impl PromotionsController {
    pub fn post(data: Json<PromotionIn>, pool: web::Data<Pool>, sender: web::Data<MessageSender>) -> ApiResult<HttpResponse> {
        let repo = Self::setup_repo(pool);
        let new_promotion = Self::build_new_promotion(data);
        Self::validate_code(&new_promotion.code)?;
        let created = repo.create(&new_promotion)?;

        sender.send(Message::PromotionCreated(created.clone()));
        Ok(HttpResponse::Created().json(created))
    }

    pub fn put(id: web::Path<i32>, data: Json<PromotionIn>, pool: web::Data<Pool>, sender: web::Data<MessageSender>) -> ApiResult<HttpResponse> {
        let repo = Self::setup_repo(pool);
        let id = id.into_inner();

        let mut promotion = repo.find(id)?;
        let PromotionIn { name, code, return_type, return_value, promotion_type, organization_id } = data.into_inner();
        promotion = Promotion { name, code: code.to_lowercase(), return_type: return_type.to_string(), return_value, type_: promotion_type.to_string(), organization_id, ..promotion };
        Self::validate_code(&promotion.code)?;
        repo.update(&promotion)?;

        sender.send(Message::PromotionUpdate(promotion.clone()));
        Ok(HttpResponse::Ok().json(promotion))
    }

    pub fn delete(id: web::Path<i32>, pool: web::Data<Pool>, sender: web::Data<MessageSender>) -> ApiResult<HttpResponse> {
        let repo = Self::setup_repo(pool);
        let id = id.into_inner();
        repo.delete(id)?;

        sender.send(Message::PromotionDeleted(id.into()));
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

