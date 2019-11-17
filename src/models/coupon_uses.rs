use crate::schema::coupon_uses;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Clone)]
#[table_name = "coupon_uses"]
pub struct CouponUses {
    coupon_id: i32,
    promotion_id: i32,
    external_user: i32
}