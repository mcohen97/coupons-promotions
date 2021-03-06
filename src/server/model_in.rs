use std::collections::HashMap;
use crate::services::{DemographyIn, RequiredAttribute};
use std::borrow::Cow;
use crate::messages::EvaluationInfo;
use crate::models::{DateTime, PromotionReturn, PromotionType};
use actix_web::web::Query;

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
    pub condition: String,
    pub return_type: ReturnTypesIn,
    pub return_value: f64,
    pub promotion_type: PromotionType,
    pub expiration: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromotionUpdateIn {
    pub code: String,
    pub name: String,
    pub condition: String,
    pub return_type: ReturnTypesIn,
    pub return_value: f64,
    pub expiration: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize)]
pub struct PaginationIn {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub offset: i64,
    pub limit: i64,
}


impl Pagination {
    pub fn get_or_default(pag: Query<PaginationIn>) -> Pagination {
        let offset = pag.offset.unwrap_or(0) as i64;
        let limit = pag.limit.unwrap_or(10) as i64;

        Pagination { offset, limit }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PromotionQueries {
    name: Option<String>,
    code: Option<String>,
    promotion_type: Option<String>,
    active: Option<bool>,
}

impl PromotionQueries {
    pub fn into_params(self) -> (Option<String>, Option<String>, Option<String>, Option<bool>) {
        (
            Self::format_param(self.name),
            Self::format_param(self.code),
            Self::format_param(self.promotion_type),
            self.active
        )
    }

    fn format_param(param: Option<String>) -> Option<String> {
        if let Some(param) = param {
            Some(format!("%{}%", param))
        } else {
            None
        }
    }
}