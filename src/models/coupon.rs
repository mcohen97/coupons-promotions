use crate::schema::coupons;
use crate::models::{DateTime, CouponUses};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Clone)]
#[table_name = "coupons"]
pub struct Coupon {
    pub id: i32,
    pub coupon_code: String,
    pub promotion_id: i32,
    pub expiration: DateTime,
    pub max_uses: i32
}

impl Coupon {
    pub fn can_keep_being_used(&self, uses: &CouponUses) -> bool {
        self.max_uses > uses.uses
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "coupons"]
pub struct NewCoupon {
    pub coupon_code: String,
    pub promotion_id: i32,
    pub expiration: DateTime,
    pub max_uses: i32
}