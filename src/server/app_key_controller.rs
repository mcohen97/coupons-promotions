use crate::server::{ApiResult, ServiceFactory, PaginationIn};
use crate::models::AppKey;
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
    pub fn post(data: Json<NewAppkeyIn>, fact: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &POST_PERMS)?;
        let service = fact.as_services()?.appkey_repo;
        let promotions = &data.promotions;
        let token = service.create(promotions, org)?;

        Ok(HttpResponse::Ok().json(AppKeyOut { token }))
    }

    pub fn get_all(fact: Data<ServiceFactory>, auth: Option<Authorization>, pag: Query<PaginationIn>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &GET_PERMS)?;
        let service = fact.as_services()?.appkey_repo;
        let pag = Pagination::get_or_default(pag);
        let res: Vec<String> = service
            .get_all(&org, pag)?
            .into_iter()
            .map(|key| key.token)
            .collect();

        Ok(HttpResponse::Ok().json(res))
    }

    pub fn get(token: Path<String>, fact: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let token = token.into_inner();
        let org = Authorization::validate(&auth, &GET_PERMS)?;
        let service = fact.as_services()?.appkey_repo;
        let promotions = service.get_promotions_by_token(&token, &org)?;

        Ok(HttpResponse::Ok().json(AppKeyOutIndividual { token, promotions }))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAppkeyIn {
    promotions: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppKeyOut {
    token: String,
}

impl From<AppKey> for AppKeyOut {
    fn from(appkey: AppKey) -> Self {
        AppKeyOut { token: appkey.token }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppKeyOutIndividual {
    token: String,
    promotions: Vec<i32>,
}