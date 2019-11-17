use crate::schema::coupons;
use crate::models::DateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Clone)]
#[table_name = "coupons"]
pub struct Coupon {
    pub id: i32,
    pub coupon_code: String,
    pub promotion_id: i32,
    pub expiration: DateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "coupons"]
pub struct NewCoupon {
    pub coupon_code: String,
    pub promotion_id: i32,
    pub expiration: DateTime,
}