use diesel::prelude::*;
use crate::schema::coupons::dsl::coupons;
use crate::schema::coupons::dsl::*;
use crate::models::{Connection, Coupon, NewCoupon};
use std::rc::Rc;
use crate::server::{ApiResult, ApiError};
use diesel::Connection as DieselConn;

#[derive(Clone)]
pub struct CouponsRepository {
    conn: Rc<Connection>
}

impl CouponsRepository {
    pub fn new(conn: Rc<Connection>) -> Self {
        CouponsRepository { conn }
    }

    pub fn create(&self, coupon: &NewCoupon) -> ApiResult<Coupon> {
        Ok(diesel::insert_into(coupons)
            .values(coupon)
            .get_result(&*self.conn)?)
    }

    pub fn create_batch(&self, batch: &[NewCoupon]) -> ApiResult<Vec<Coupon>> {
        self.conn.transaction(|| {
            let result = batch.into_iter()
                .map(|coupon| self.create(coupon))
                .collect();

            result
        })
    }

    pub fn find(&self, promotion: i32, coupon: &str) -> ApiResult<Coupon> {
        let split: Vec<&str> = coupon.split('#').collect();
        if split.len() != 2 {
            return Err(ApiError::from("Invalid coupon code"));
        }
        let _code = split[0].to_string();
        let _id: i32 = split[1].parse().map_err( |_| ApiError::from("Invalid coupon code"))?;

        Ok(coupons.find((_id, promotion))
            .first(&*self.conn)?
        )
    }

    pub fn get_by_promotion(&self, promotion: i32) -> ApiResult<Vec<Coupon>> {
        Ok(coupons
            .filter(promotion_id.eq(&promotion))
            .load(&*self.conn)?)
    }
}
