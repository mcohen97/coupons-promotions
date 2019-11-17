use crate::server::ApiResult;
use std::collections::HashMap;
use crate::models::{Promotion, PromotionRepository, PromotionType, PromotionExpression, PromotionReturn, CouponsRepository, CouponUsesRepository, Coupon, TransactionRepository, Transaction};
use crate::server::ApiError;
use chrono::Utc;
use crate::messages::MessageSender;
use std::sync::Arc;
use crate::services::EvaluationSpecificDto;

#[derive(Clone)]
pub struct EvaluationServices {
    promotions_repo: PromotionRepository,
    coupon_repo: CouponsRepository,
    coupon_uses_repo: CouponUsesRepository,
    transaction_repo:  TransactionRepository,
    message_sender: Arc<MessageSender>,
}

impl EvaluationServices {
    pub fn new(promotions_repo: PromotionRepository, coupon_repo: CouponsRepository, coupon_uses_repo: CouponUsesRepository, transaction_repo:  TransactionRepository,message_sender: Arc<MessageSender>) -> Self {
        Self { promotions_repo, message_sender, coupon_uses_repo, coupon_repo, transaction_repo }
    }

    pub fn evaluate_promotion(&self, promotion_id: i32, specific_data: EvaluationSpecificDto, attributes: HashMap<String, f64>) -> ApiResult<EvaluationResultDto> {
        let promotion = self.promotions_repo.find(promotion_id)?;
        self.validate_promotion_is_active(&promotion)?;
        self.validate_specific_data(&promotion, &specific_data)?;
        self.validate_not_expires(&promotion)?;

        let total = attributes.get("total").map(|v| v.to_owned());
        let return_type = &promotion.return_type;
        let expr = PromotionExpression::parse(&promotion.code)?;
        let eval_result = expr.evaluate(attributes)?;

        let organization_id = promotion.organization_id;
        let res = if eval_result {
            self.after_successful_evaluation_update(promotion.clone(), &specific_data)?;
            EvaluationResultDto::Applies {
                organization_id,
                return_type: return_type.to_string(),
                total_discount: self.calculate_total_discount(total, promotion.get_return())?,
            }
        } else {
            EvaluationResultDto::DoesntApply { organization_id }
        };

        Ok(res)
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

    fn validate_specific_data(&self, promotion: &Promotion, specific_data: &EvaluationSpecificDto) -> ApiResult<()> {
        match specific_data {
            EvaluationSpecificDto::Discount { transaction_id } => { // TODO: Add transaction id validation
                if let PromotionType::Coupon = promotion.get_type() {
                    return Err(ApiError::from("Promotion type specific data doesnt match with promotion type"));
                }
                if self.transaction_repo.exists(*transaction_id)? {
                    return Err(ApiError::from("Transaction id has already been used"));
                }
                Ok(())
            }
            EvaluationSpecificDto::Coupon { user, coupon_code } => {
                if let PromotionType::Discount = promotion.get_type() {
                    return Err(ApiError::from("Promotion type specific data doesnt match with promotion type"));
                }
                let coupon = self.get_coupon(promotion.id, &coupon_code)?;
                self.validate_coupon_has_uses(&coupon, *user)
            }
        }
    }

    fn get_coupon(&self, promotion_id: i32, coupon_code: &str) -> ApiResult<Coupon> {
        self.coupon_repo.find(promotion_id, coupon_code)
    }

    fn validate_coupon_has_uses(&self, coupon: &Coupon, user: i32) -> ApiResult<()> {
        let uses = self.coupon_uses_repo.find_or_create(coupon.promotion_id, coupon.id, user)?;
        if coupon.can_keep_being_used(&uses) {
            Ok(())
        } else {
            Err(ApiError::from(format!("User {} has reached their uses limit", user)))
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

    fn after_successful_evaluation_update(&self, promotion: Promotion, specific_data: &EvaluationSpecificDto) -> ApiResult<()> {
        match specific_data {
            EvaluationSpecificDto::Discount { transaction_id } => {
                // self.deactivate_promotion(promotion)
                self.transaction_repo.create(&Transaction {id: *transaction_id})?;
                Ok(())
            },
            EvaluationSpecificDto::Coupon { user, coupon_code } => {
                let coupon = self.get_coupon(promotion.id, &coupon_code)?;
                let mut uses = self.coupon_uses_repo.find_or_create(coupon.promotion_id, coupon.id, *user)?;
                uses.uses += 1;
                self.coupon_uses_repo.update(&uses)?;
                Ok(())
            }
        }
    }

    fn deactivate_promotion(&self, mut promotion: Promotion) -> ApiResult<()> {
        promotion.active = false;
        self.promotions_repo.update(&promotion)
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

