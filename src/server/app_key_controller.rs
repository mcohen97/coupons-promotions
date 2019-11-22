use crate::server::{ApiResult, ServiceFactory};
use actix_web::HttpResponse;
use actix_web::web::{Json, Data};
use crate::server::authenticater::Authorization;

lazy_static! {
    static ref POST_PERMS: Vec<&'static str> = vec!["ADMIN"];
}

pub struct AppKeyController;
impl AppKeyController {
    pub fn post(data: Json<NewAppkeyIn>, fact: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        Authorization::validate(&auth, &POST_PERMS)?;
        let service = fact.as_services()?.appkey_repo;
        let promotions = &data.promotions;
        let token = service.create(promotions)?;

        Ok(HttpResponse::Ok().json(AppKeyOut { token }))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewAppkeyIn {
    promotions: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppKeyOut {
    token: String
}