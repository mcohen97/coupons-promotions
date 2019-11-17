use crate::models::{NewCoupon, DateTime, Coupon};

#[derive(Serialize, Deserialize)]
pub struct GenerateCouponsDto {
    pub promotion_id: i32,
    pub coupon_code: String,
    pub quantity: u32,
    pub expiration: DateTime,
    pub max_uses: i32
}

impl GenerateCouponsDto {
    pub fn generate(&self) -> NewCoupon {
        NewCoupon {
            coupon_code: self.coupon_code.clone(),
            promotion_id: self.promotion_id,
            expiration: self.expiration,
            max_uses: self.max_uses
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CouponsDto {
    pub promotion_id: i32,
    pub coupon_code: String,
    pub expiration: DateTime,
}

impl From<Coupon> for CouponsDto {
    fn from(c: Coupon) -> Self {
        CouponsDto {
            coupon_code: format!("{}#{}", c.coupon_code, c.id),
            expiration: c.expiration,
            promotion_id: c.promotion_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvaluationSpecificDto {
    Discount { transaction_id: i32 },
    Coupon { user: i32, coupon_code: String }
}