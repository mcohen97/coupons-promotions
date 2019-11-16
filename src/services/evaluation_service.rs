use crate::server::ApiResult;
use std::collections::HashMap;
use crate::models::{Promotion, PromotionRepository, PromotionType, PromotionExpression, PromotionReturn};
use crate::server::ApiError;
use chrono::Utc;
use crate::messages::MessageSender;
use std::sync::Arc;

#[derive(Clone)]
pub struct EvaluationServices {
    promotions_repo: PromotionRepository,
    message_sender: Arc<MessageSender>,
}

impl EvaluationServices {
    pub fn new(repo: PromotionRepository, message_sender: Arc<MessageSender>) -> Self {
        Self { promotions_repo: repo, message_sender }
    }

    pub fn evaluate_promotion(&self, promotion_id: i32, required: RequiredAttribute, attributes: HashMap<String, f64>) -> ApiResult<EvaluationResultDto> {
        let promotion = self.promotions_repo.find(promotion_id)?;
        self.validate_promotion_is_active(&promotion)?;
        self.validate_required_attribute(&promotion, required)?;
        self.validate_not_expires(&promotion)?;

        let total = attributes.get("total").map(|v| v.to_owned());
        let return_type = &promotion.return_type;
        let expr = PromotionExpression::parse(&promotion.code)?;
        let eval_result = expr.evaluate(attributes)?;

        let organization_id = promotion.organization_id;
        Ok(match eval_result {
            true => EvaluationResultDto::Applies { organization_id, return_type: return_type.to_string(), total_discount: self.calculate_total_discount(total, promotion.get_return())? },
            false => EvaluationResultDto::DoesntApply { organization_id }
        })
    }

    fn validate_not_expires(&self, promotion: &Promotion) -> ApiResult<()> {
        let now = Utc::now();
        let expiration_has_passed = now > promotion.expiration;
        if expiration_has_passed {
            let diff = now - promotion.expiration;
            Err(ApiError::BadRequest(format!("Coupon expired {} hours ago", diff.num_hours()).into()))
        } else {
            Ok(())
        }
    }

    fn validate_promotion_is_active(&self, promotion: &Promotion) -> ApiResult<()> {
        if !promotion.active {
            Err(ApiError::BadRequest("Promotion is not active".into()))
        } else {
            Ok(())
        }
    }

    fn validate_organization_exists(&self, org_id: i32) -> ApiResult<()> {
        self.promotions_repo.find(org_id).map(|_| ())
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationResultDto {
    Applies { organization_id: i32, total_discount: f64, return_type: String },
    DoesntApply { organization_id: i32 },
}

