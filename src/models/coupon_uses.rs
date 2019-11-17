use crate::schema::coupon_uses;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Clone)]
#[table_name = "coupon_uses"]
pub struct CouponUses {
    pub coupon_id: i32,
    pub promotion_id: i32,
    pub external_user: i32,
    pub uses: i32,
}