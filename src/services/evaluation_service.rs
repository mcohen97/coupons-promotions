use std::collections::HashMap;
use crate::server::*;

pub struct EvaluationService;

impl EvaluationService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, code: String, attributes: HashMap<String, f64>, app_key: &str) -> ApiResult<EvaluationResult> {
        let _c = code;
        let _a = attributes;
        let _res = EvaluationResult {};
        let _key = app_key;
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationResult {

}