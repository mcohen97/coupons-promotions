use crate::schema::promotions;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "promotions"]
pub struct Promotion {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub active: bool,
    pub return_type: i32,
    pub return_value: i32,
    pub type_: String,
    pub organization_id: i32,
    pub invocations: i32,
    pub negative_responses: i32,
    pub average_response_time: f64,
    pub total_spent: f64,
}

pub enum PromotionType {
    Discount,
    Coupon,
    Other,
}

impl Promotion {
    pub fn get_type(&self) -> PromotionType {
        match self.type_.as_ref() {
            "discount" => PromotionType::Discount,
            "coupon" => PromotionType::Coupon,
            _ => PromotionType::Other
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EvaluationResult {
    PromotionDoesntApply,
    PromotionApplies { discount_type: String },
}

