use crate::server::{ApiResult, ServiceFactory};
use actix_web::{HttpResponse, web};
use actix_web::web::{Data, Json};
use crate::services::GenerateCouponsDto;
use crate::models::DateTime;
use crate::server::authenticater::Authorization;

lazy_static! {
    static ref GET_PERMS: Vec<&'static str> = vec!["ADMIN", "GET_PROMOTIONS"];
    static ref POST_PERMS: Vec<&'static str> = vec!["ADMIN"];
}

pub struct CouponsController;
impl CouponsController {
    pub fn post(id: web::Path<i32>, data: Json<GenerateCouponsIn>, factory: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &POST_PERMS)?;
        let service = factory.as_services()?.coupons;
        let id = id.into_inner();
        let res = service.generate_coupons(data.into_inner().into_dto(id), org)?;

        Ok(HttpResponse::Ok().json(res))
    }

    pub fn get(id: web::Path<i32>, factory: Data<ServiceFactory>, auth: Option<Authorization>) -> ApiResult<HttpResponse> {
        let org = Authorization::validate(&auth, &GET_PERMS)?;
        let service = factory.as_services()?.coupons;
        let id = id.into_inner();
        let res = service.get_coupons(id, org)?;

        Ok(HttpResponse::Ok().json(res))
    }
}

#[derive(Serialize, Deserialize)]
pub struct GenerateCouponsIn {
    pub coupon_code: String,
    pub quantity: u32,
    pub expiration: DateTime,
    pub max_uses: i32,
}

impl GenerateCouponsIn {
    pub fn into_dto(self, promotion_id: i32) -> GenerateCouponsDto {
        GenerateCouponsDto {
            promotion_id,
            coupon_code: self.coupon_code,
            expiration: self.expiration,
            quantity: self.quantity,
            max_uses: self.max_uses,
        }
    }
}
