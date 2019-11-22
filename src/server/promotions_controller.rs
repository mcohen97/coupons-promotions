use actix_web::{web, HttpResponse};
use crate::server::{ApiResult, ServiceFactory};
use actix_web::web::{Json, Data};

use crate::server::model_in::PromotionIn;
use crate::server::authenticater::Authorization;

pub struct PromotionsController;

lazy_static! {
    static ref GET_PERMS: Vec<&'static str> = vec!["ADMIN","GET_PROMOTIONS"];
    static ref POST_PERMS: Vec<&'static str> = vec!["ADMIN"];
    static ref DELETE_PERMS: Vec<&'static str> = vec!["ADMIN"];
    static ref PUT_PERMS: Vec<&'static str> = vec!["ADMIN"];
}

impl PromotionsController {
    pub fn post(data: Json<PromotionIn>, services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        Authorization::validate(&auth, &POST_PERMS)?;
        let service = services.as_services()?.promotions;
        let created = service.create(data.into_inner())?;

        Ok(HttpResponse::Created().json(created))
    }

    pub fn put(id: web::Path<i32>, data: Json<PromotionIn>, services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        Authorization::validate(&auth, &PUT_PERMS)?;
        let service = services.as_services()?.promotions;
        let updated = service.update(id.into_inner(), data.into_inner())?;

        Ok(HttpResponse::Ok().json(updated))
    }

    pub fn delete(id: web::Path<i32>, services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        Authorization::validate(&auth, &DELETE_PERMS)?;
        let service = services.as_services()?.promotions;
        service.delete(id.into_inner())?;

        Ok(HttpResponse::Ok().finish())
    }

    pub fn get(id: web::Path<i32>, services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        Authorization::validate(&auth, &GET_PERMS)?;
        let service = services.as_services()?.promotions;
        let promotion = service.get(id.into_inner())?;

        Ok(HttpResponse::Ok().json(&promotion))
    }

    pub fn get_all(services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        Authorization::validate(&auth, &GET_PERMS)?;
        let service = services.as_services()?.promotions;
        let promotion = service.get_all()?;

        Ok(HttpResponse::Ok().json(&promotion))
    }
}