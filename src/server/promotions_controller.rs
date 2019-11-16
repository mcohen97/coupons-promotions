use actix_web::{web, HttpResponse};
use crate::server::{ApiResult, ServiceFactory, PromotionIn};
use actix_web::web::{Json, Data};

pub struct PromotionsController;

impl PromotionsController {
    pub fn post(data: Json<PromotionIn>, services: Data<ServiceFactory>) -> ApiResult<HttpResponse> {
        let service = services.as_services()?.promotions;
        let created = service.create(data.into_inner())?;

        Ok(HttpResponse::Created().json(created))
    }

    pub fn put(id: web::Path<i32>, data: Json<PromotionIn>, services: Data<ServiceFactory>) -> ApiResult<HttpResponse> {
        let service = services.as_services()?.promotions;
        let updated = service.update(id.into_inner(), data.into_inner())?;

        Ok(HttpResponse::Ok().json(updated))
    }

    pub fn delete(id: web::Path<i32>, services: Data<ServiceFactory>) -> ApiResult<HttpResponse> {
        let service = services.as_services()?.promotions;
        service.delete(id.into_inner())?;

        Ok(HttpResponse::Ok().finish())
    }

    pub fn get(id: web::Path<i32>, services: Data<ServiceFactory>) -> ApiResult<HttpResponse> {
        let service = services.as_services()?.promotions;
        let promotion = service.get(id.into_inner())?;

        Ok(HttpResponse::Ok().json(&promotion))
    }

    pub fn get_all(services: Data<ServiceFactory>) -> ApiResult<HttpResponse> {
        let service = services.as_services()?.promotions;
        let promotion = service.get_all()?;

        Ok(HttpResponse::Ok().json(&promotion))
    }
}

