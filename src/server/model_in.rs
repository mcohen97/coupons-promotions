use std::collections::HashMap;
use crate::services::{DemographyIn, RequiredAttribute};
use std::borrow::Cow;
use crate::messages::EvaluationInfo;
use crate::models::{DateTime, PromotionReturn, PromotionType};

#[derive(Serialize, Deserialize)]
pub struct EvaluationIn {
    pub attributes: HashMap<String, f64>,
    pub demographic_data: Option<DemographyIn>,
    #[serde(flatten)]
    pub required: RequiredAttribute,
}

#[derive(Serialize)]
pub struct EvaluationOut {
    pub promotion_id: i32,
    pub organization_id: i32,
    pub evaluation_info: EvaluationInfo,
    pub demographic_data: Cow<'static, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromotionIn {
    pub code: String,
    pub name: String,
    pub return_type: ReturnTypesIn,
    pub return_value: f64,
    pub promotion_type: PromotionType,
    pub organization_id: i32,
    pub expiration: DateTime,
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
        match self {
            ReturnTypesIn::Percentage => "percentage".to_string(),
            ReturnTypesIn::Fixed => "fixed".to_string()
        }
    }
}