#[allow(unused_variables)]
use crate::server::ApiResult;
use std::collections::HashMap;
use crate::models::{Connection, Promotion, PromotionRepo};

pub struct EvaluationService<'a> {
    repo: PromotionRepo<'a>
}

impl<'a> EvaluationService<'a> {
    pub fn new(repo: PromotionRepo<'a>) -> Self {
        Self { repo }
    }

    pub fn evaluate_promotion(&self, promotion_id: i32, attributes: HashMap<String, f64>) -> ApiResult<EvaluationResult> {
        let promotion = self.repo.find(promotion_id)?;
        
        unimplemented!()
    }
}

pub enum EvaluationResult {
    Valid { ret_type: String, value: f64 },
    Invalid
}

