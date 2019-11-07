#[allow(unused_variables)]
use crate::server::ApiResult;
use std::collections::HashMap;
use crate::models::{Promotion, PromotionRepo, PromotionType, PromotionExpression, PromotionReturn};
use crate::server::ApiError;

pub struct EvaluationService {
    repo: Box<PromotionRepo>
}

impl EvaluationService {
    pub fn new(repo: Box<PromotionRepo>) -> Self {
        Self { repo }
    }

    pub fn evaluate_promotion(&self, promotion_id: i32, required: RequiredAttribute, attributes: HashMap<String, f64>) -> ApiResult<EvaluationResult> {
        let promotion = self.repo.find(promotion_id)?;
        self.validate_required_attribute(&promotion, required)?;
        let total = attributes.get("total".into()).map(|v| v.to_owned());
        let expr = PromotionExpression::parse(&promotion.code)?;
        let eval_result = expr.evaluate(attributes)?;

        let organization_id = promotion.organization_id;
        Ok(match eval_result {
            true => EvaluationResult::Applies { organization_id, total_discount: self.calculate_total_discount(total, promotion.get_return())? },
            false => EvaluationResult::DoesntApply { organization_id }
        })
    }

    fn validate_required_attribute(&self, promotion: &Promotion, required: RequiredAttribute) -> ApiResult<()> {
        match promotion.get_type() {
            PromotionType::Discount => {
                if let RequiredAttribute::TransactionId(_) = required {
                    Ok(())
                } else {
                    Err(ApiError::from("Missing transaction id"))
                }
            }
            PromotionType::Coupon => {
                if let RequiredAttribute::CouponCode(_) = required {
                    Ok(())
                } else {
                    Err(ApiError::from("Missing coupon code"))
                }
            }
        }
    }

    fn calculate_total_discount(&self, total: Option<f64>, p_return: PromotionReturn) -> ApiResult<f64> {
        Ok(match p_return {
            PromotionReturn::Fixed(discount) => discount,
            PromotionReturn::Percentage(percentage) => {
                let total = total.ok_or(ApiError::from("Missing total attribute"))?;
                total * (percentage / 100.0)
            }
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredAttribute {
    TransactionId(u32),
    CouponCode(u32),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationResult {
    Applies { organization_id: i32, total_discount: f64 },
    DoesntApply { organization_id: i32 },
}

