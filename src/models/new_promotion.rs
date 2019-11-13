use crate::schema::promotions;
use crate::models::{PromotionReturn, PromotionType};
use chrono::NaiveDate;

#[derive(Insertable, Deserialize)]
#[table_name = "promotions"]
pub struct NewPromotion {
    pub code: String,
    pub name: String,
    pub active: bool,
    pub return_type: String,
    pub return_value: f64,
    pub type_: String,
    pub organization_id: i32,
    pub expiration: NaiveDate,
}

impl NewPromotion {
    pub fn new(
        name: String,
        code: String,
        active: bool,
        p_return: PromotionReturn,
        p_type: PromotionType,
        organization_id: i32,
        expiration: NaiveDate,
    ) -> Self {
        let (return_type, return_value) = match p_return {
            PromotionReturn::Percentage(val) => ("percentage".into(), val),
            PromotionReturn::Fixed(val) => ("fixed".into(), val)
        };
        let type_ = match p_type {
            PromotionType::Coupon => "coupon".into(),
            PromotionType::Discount => "discount".into()
        };

        NewPromotion { name, code, active, return_value, return_type, type_, organization_id, expiration }
    }
}