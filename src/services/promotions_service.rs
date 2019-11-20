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

    pub fn get(&self, id: i32) -> ApiResult<Promotion> {
        let promo = self.promotions_repo.find(id)?;

        Ok(promo)
    }

    pub fn get_all(&self) -> ApiResult<Vec<Promotion>> {
        let promos = self.promotions_repo.get()?;

        Ok(promos)
    }

    pub fn create(&self, promotion: PromotionIn) -> ApiResult<Promotion> {
        self.validate_organization_exists(promotion.organization_id)?;
        let new_promotion = Self::build_new_promotion(promotion);

        let created = self.promotions_repo.create(&new_promotion)?;
        self.message_sender.send(Message::PromotionCreated(created.clone()));

        Ok(created)
    }

    fn build_new_promotion(data: PromotionIn) -> NewPromotion {
        let PromotionIn { name, code, return_type, return_value, promotion_type, organization_id, expiration } = data;
        let ret = return_type.get_return(return_value);
        NewPromotion::new(
            name,
            code.to_lowercase(),
            true,
            ret,
            promotion_type,
            organization_id,
            expiration,
        )
    }

    pub fn update(&self, id: i32, data: PromotionIn) -> ApiResult<Promotion> {
        let mut promotion = self.promotions_repo.find(id)?;
        self.validate_organization_exists(promotion.organization_id)?;

        let PromotionIn { name, code, return_type, return_value, promotion_type, organization_id, expiration } = data;
        promotion = Promotion { name, code: code.to_lowercase(), return_type: return_type.to_string(), return_value, type_: promotion_type.to_string(), organization_id, expiration, ..promotion };
        self.promotions_repo.update(&promotion)?;

        self.message_sender.send(Message::PromotionUpdate(promotion.clone()));
        Ok(promotion)
    }

    pub fn delete(&self, id: i32) -> ApiResult<()> {
        self.promotions_repo.delete(id)?;
        self.message_sender.send(Message::PromotionDeleted(id.into()));

        Ok(())
    }

    fn validate_organization_exists(&self, organization: i32) -> ApiResult<()> {
        let exists = self.organization_repo.exists(organization)?;

        if !exists {
            Err(ApiError::BadRequest("Organization doesnt exists".to_string().into()))
        } else {
            Ok(())
        }
    }
}