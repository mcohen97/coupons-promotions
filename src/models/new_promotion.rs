use crate::schema::promotions;
use crate::models::{PromotionReturn, PromotionType};

#[derive(Insertable)]
#[table_name="promotions"]
pub struct NewPromotion<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub active: bool,
    pub return_type: &'a str,
    pub return_value: f64,
    pub type_: &'a str,
    pub organization_id: i32,
}

impl<'a> NewPromotion<'a> {
    pub fn new(
        name: &'a str,
        code: &'a str,
        active: bool,
        p_return: PromotionReturn,
        p_type: PromotionType,
        organization_id: i32, ) -> Self {
        let (return_type, return_value) = match p_return {
            PromotionReturn::Percentage(val) => ("percentage".into(), val),
            PromotionReturn::Fixed(val) => ("fixed".into(), val)
        };
        let type_ = match p_type {
            PromotionType::Coupon => "coupon".into(),
            PromotionType::Discount => "discount".into()
        };

        NewPromotion { name, code, active, return_value, return_type, type_, organization_id}
    }
}