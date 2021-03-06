use crate::server::{ApiResult, ServiceFactory, PaginationIn};
use crate::models::AppKeyOut;
use actix_web::HttpResponse;
use actix_web::web::{Json, Data, Path};
use crate::server::authenticater::Authorization;
use crate::server::model_in::Pagination;
use actix_web::web::Query;

lazy_static! {
    static ref GET_PERMS: Vec<&'static str> = vec!["ADMIN"];
    static ref POST_PERMS: Vec<&'static str> = vec!["ADMIN"];
}

pub struct AppKeyController;

impl AppKeyController {
    pub fn get(token: Path<String>, fact: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let token = token.into_inner();
        let org = Authorization::validate(&auth, &GET_PERMS)?;
        let service = fact.as_services()?.appkey_repo;
        let promotions = service.get_promotions_by_token(&token, &org)?;
        let promo_names = service.get_promotions_codes_from_ids(&promotions)?;
        let name = service.get_name(&token, &org)?;

        Ok(HttpResponse::Ok().json(AppKeyOut { token, promotion_ids: promotions, organization_id: org, name, promotion_names: promo_names }))
    }

    pub fn get_all(fact: Data<ServiceFactory>, auth: Option<Authorization>, pag: Query<PaginationIn>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &GET_PERMS)?;
        let service = fact.as_services()?.appkey_repo;
        let pag = Pagination::get_or_default(pag);
        let res: Vec<AppKeyOut> = service
            .get_all(&org, pag)?;

        Ok(HttpResponse::Ok().json(res))
    }

    pub fn post(data: Json<NewAppkeyIn>, fact: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &POST_PERMS)?;
        let service = fact.as_services()?.appkey_repo;
        let NewAppkeyIn { promotions, name } = data.into_inner();
        let token = service.create(&promotions, org, name)?;

        Ok(HttpResponse::Ok().json(token))
    }

    pub fn put(data: Json<NewAppkeyIn>, token: Path<String>, fact: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &POST_PERMS)?;
        let NewAppkeyIn {name, promotions} = data.into_inner();
        let service = fact.as_services()?.appkey_repo;
        let res = service.update(&name, &token.into_inner(),&org, promotions)?;

        Ok(HttpResponse::Ok().json(res))
    }

    pub fn delete(token: Path<String>, fact: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &POST_PERMS)?;
        let service = fact.as_services()?.appkey_repo;
        service.delete_token(&token.into_inner(), &org)?;

        Ok(HttpResponse::Ok().finish())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAppkeyIn {
    name: String,
    promotions: Vec<i32>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppKeyOutIndividual {
    token: String,
    promotions: Vec<i32>,
}