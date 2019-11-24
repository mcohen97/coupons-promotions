use crate::models::{PromotionRepository, OrganizationRepository, NewPromotion, Promotion};
use crate::server::{ApiResult, ApiError, PromotionIn};
use crate::messages::{MessageSender, Message};

pub struct PromotionService {
    promotions_repo: PromotionRepository,
    organization_repo: OrganizationRepository,
    message_sender: MessageSender,
}

impl PromotionService {
    pub fn new(promotions_repo: PromotionRepository, organization_repo: OrganizationRepository, message_sender: MessageSender) -> Self {
        PromotionService { promotions_repo, organization_repo, message_sender }
    }

    pub fn get(&self, id: i32, org: String) -> ApiResult<Promotion> {
        let promo = self.promotions_repo.find(id, &org)?;

        Ok(promo)
    }

    pub fn get_all(&self, offset: u64, limit: u64, org: String) -> ApiResult<Vec<Promotion>> {
        let offset = offset as i64;
        let limit = limit as i64;
        let promos = self.promotions_repo.get(offset, limit, &org)?;

        Ok(promos)
    }

    pub fn create(&self, promotion: PromotionIn, org: String) -> ApiResult<Promotion> {
        self.validate_organization_exists(&org)?;
        let new_promotion = Self::build_new_promotion(promotion, org);
        let created = self.promotions_repo.create(&new_promotion)?;
        self.message_sender.send(Message::PromotionCreated(created.clone()));
        Ok(created)
    }

    fn build_new_promotion(data: PromotionIn, org: String) -> NewPromotion {
        let PromotionIn { name, code, return_type, return_value, promotion_type, expiration, condition } = data;
        let ret = return_type.get_return(return_value);
        NewPromotion::new(
            name,
            code.to_lowercase(),
            condition,
            true,
            ret,
            promotion_type,
            org,
            expiration,
        )
    }

    pub fn update(&self, id: i32, data: PromotionIn, org: String) -> ApiResult<Promotion> {
        let mut promotion = self.promotions_repo.find(id, &org)?;
        self.validate_organization_exists(&promotion.organization_id)?;

        let PromotionIn { name, code, return_type, return_value, promotion_type, expiration, condition } = data;
        promotion = Promotion { name, code: code.to_lowercase(), return_type: return_type.to_string(), return_value, type_: promotion_type.to_string(), organization_id: org, expiration, condition, ..promotion };
        self.promotions_repo.update(&promotion)?;

        self.message_sender.send(Message::PromotionUpdate(promotion.clone()));
        Ok(promotion)
    }

    pub fn delete(&self, id: i32, org: String) -> ApiResult<()> {
        self.promotions_repo.delete(id, &org)?;
        self.message_sender.send(Message::PromotionDeleted(id.into()));

        Ok(())
    }

    fn validate_organization_exists(&self, organization: &str) -> ApiResult<()> {
        let exists = self.organization_repo.exists(organization)?;

        if !exists {
            Err(ApiError::BadRequest("Organization doesnt exists".to_string().into()))
        } else {
            Ok(())
        }
    }
}