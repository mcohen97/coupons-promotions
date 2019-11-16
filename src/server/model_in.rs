use std::collections::HashMap;
use crate::services::{DemographyIn, RequiredAttribute};
use std::borrow::Cow;
use crate::messages::EvaluationInfo;



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