use actix_web::{web, HttpResponse};
use crate::server::{ApiResult, ServiceFactory, PaginationIn, PromotionQueries, PromotionUpdateIn};
use actix_web::web::{Json, Data};

use crate::server::model_in::PromotionIn;
use crate::server::authenticater::Authorization;
use actix_web::web::Query;
use crate::server::model_in::Pagination;

pub struct PromotionsController;

lazy_static! {
    static ref GET_PERMS: Vec<&'static str> = vec!["ADMIN","GET_PROMOTIONS"];
    static ref POST_PERMS: Vec<&'static str> = vec!["ADMIN"];
    static ref DELETE_PERMS: Vec<&'static str> = vec!["ADMIN"];
    static ref PUT_PERMS: Vec<&'static str> = vec!["ADMIN"];
}

impl PromotionsController {
    pub fn get_all(services: Data<ServiceFactory>, auth: Option<Authorization>, pag: Query<PaginationIn>, query: Query<PromotionQueries>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &GET_PERMS)?;
        let pag = Pagination::get_or_default(pag);
        let service = services.as_services()?.promotions;
        let promotion = service.get_all(org, pag, query.into_inner())?;

        Ok(HttpResponse::Ok().json(&promotion))
    }

    pub fn get(id: web::Path<i32>, services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &GET_PERMS)?;
        let service = services.as_services()?.promotions;
        let promotion = service.get(id.into_inner(), org)?;

        Ok(HttpResponse::Ok().json(&promotion))
    }

    pub fn post(data: Json<PromotionIn>, services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &POST_PERMS)?;
        let service = services.as_services()?.promotions;
        let created = service.create(data.into_inner(), org)?;

        Ok(HttpResponse::Created().json(created))
    }

    pub fn put(id: web::Path<i32>, data: Json<PromotionUpdateIn>, services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &PUT_PERMS)?;
        let service = services.as_services()?.promotions;
        let updated = service.update(id.into_inner(), data.into_inner(), org)?;

        Ok(HttpResponse::Ok().json(updated))
    }

    pub fn delete(id: web::Path<i32>, services: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &DELETE_PERMS)?;
        let service = services.as_services()?.promotions;
        service.delete(id.into_inner(), org)?;

        Ok(HttpResponse::Ok().finish())
    }
}