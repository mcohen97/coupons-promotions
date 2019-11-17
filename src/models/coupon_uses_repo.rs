use crate::schema::coupon_uses::dsl::coupon_uses;
use crate::schema::coupon_uses::*;
use diesel::prelude::*;
use crate::models::{Connection, CouponUses};
use std::rc::Rc;
use crate::server::ApiResult;

#[derive(Clone)]
pub struct CouponUsesRepository {
    conn: Rc<Connection>
}

impl CouponUsesRepository {
    pub fn new(conn: Rc<Connection>) -> Self {
        CouponUsesRepository { conn }
    }

    pub fn find_or_create(&self, promotion: i32, coupon: i32, user: i32) -> ApiResult<CouponUses> {
        let found = coupon_uses.find((coupon, promotion, user)).first::<CouponUses>(&*self.conn);
        if let Err(diesel::NotFound) = found {
            let c_uses = CouponUses { uses: 0, promotion_id: promotion, coupon_id: coupon, external_user: user };
            let created = self.create(&c_uses)?;

            Ok(created)
        } else {
            Ok(found?)
        }
    }

    pub fn create(&self, c_uses: &CouponUses) -> ApiResult<CouponUses> {
        Ok(diesel::insert_into(coupon_uses).values(c_uses).get_result(&*self.conn)?)
    }

    pub fn add_use(&self, c_uses: &CouponUses) -> ApiResult<CouponUses> {
        let find = coupon_uses.find((c_uses.coupon_id, c_uses.promotion_id, c_uses.external_user));
        Ok(diesel::update(find)
            .set(uses.eq(uses + 1))
            .get_result(&*self.conn)?
        )
    }
}