mod demography_service;
mod evaluation_service;
mod promotions_service;
mod coupon_services;


pub use demography_service::*;
pub use evaluation_service::*;
pub use promotions_service::*;
pub use coupon_services::*;
use crate::models::{Connection, PromotionRepository, OrganizationRepository};
use std::rc::Rc;
use crate::messages::MessageSender;
use std::sync::Arc;

pub struct Services {
    pub message_sender: Arc<MessageSender>,
    pub evaluation: EvaluationServices,
    pub demographic: DemographyServices,
    pub promotions: PromotionService,
    pub promotions_repo: PromotionRepository,
    pub organizations_repo: OrganizationRepository,
}

impl Services {
    pub fn new(conn: Connection, message_sender: Arc<MessageSender>) -> Services {
        let conn = Rc::new(conn);
        let organizations = OrganizationRepository::new(conn.clone());
        let promotions_repo = PromotionRepository::new(conn.clone());
        let evaluation = EvaluationServices::new(promotions_repo.clone());
        let demographic = DemographyServices::new();
        let promotions = PromotionService::new(promotions_repo.clone(), organizations.clone(), message_sender.clone());
        Services {
            evaluation,
            demographic,
            organizations_repo: organizations,
            promotions_repo,
            message_sender,
            promotions
        }
    }
}