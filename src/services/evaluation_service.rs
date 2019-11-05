#[allow(unused_variables)]
use crate::server::ApiResult;
use std::collections::HashMap;
use crate::models::{Connection, Promotion};

pub struct EvaluationService {
    conn: Connection
}

impl EvaluationService {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn evaluate_promotion(&self, promotion_id: i32, attributes: HashMap<String, f64>) -> ApiResult<EvaluationResult> {
        unimplemented!()
    }
}

pub enum EvaluationResult {
    Valid { ret_type: String, value: f64 },
    Invalid
}

