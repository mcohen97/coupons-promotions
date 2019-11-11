use crate::models::{PromotionType, PromotionReturn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromotionIn {
    pub code: String,
    pub name: String,
    pub return_type: ReturnTypesIn,
    pub return_value: f64,
    pub promotion_type: PromotionType,
    pub organization_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReturnTypesIn {
    Percentage,
    Fixed,
}

impl ReturnTypesIn {
    pub fn get_return(&self, value: f64) -> PromotionReturn {
        match self {
            ReturnTypesIn::Percentage => PromotionReturn::Percentage(value),
            ReturnTypesIn::Fixed => PromotionReturn::Fixed(value)
        }
    }
}

impl ReturnTypesIn {
    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}