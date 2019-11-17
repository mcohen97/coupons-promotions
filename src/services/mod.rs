mod demography_service;
mod evaluation_service;
mod promotions_service;
mod coupon_services;
mod dtos;

pub use demography_service::*;
pub use evaluation_service::*;
pub use promotions_service::*;
pub use coupon_services::*;
pub use dtos::*;
use crate::models::{Connection, PromotionRepository, OrganizationRepository, CouponsRepository, CouponUsesRepository, TransactionRepository};
use std::rc::Rc;
use crate::messages::MessageSender;
use std::sync::Arc;

pub struct Services {
    pub message_sender: Arc<MessageSender>,
    pub evaluation: EvaluationServices,
    pub demographic: DemographyServices,
    pub promotions: PromotionService,
    pub coupons: CouponServices,
    pub promotions_repo: PromotionRepository,
    pub organizations_repo: OrganizationRepository,
    pub coupons_repo: CouponsRepository,
    pub coupon_uses_repo: CouponUsesRepository,
    pub transaction_repo: TransactionRepository,
}

impl Services {
    pub fn new(conn: Connection, message_sender: Arc<MessageSender>) -> Services {
        let conn = Rc::new(conn);
        let organizations = OrganizationRepository::new(conn.clone());
        let promotions_repo = PromotionRepository::new(conn.clone());
        let transaction_repo: TransactionRepository = TransactionRepository::new(conn.clone());
        let coupons_repo = CouponsRepository::new(conn.clone());
        let coupon_uses_repo = CouponUsesRepository::new(conn.clone());
        let evaluation = EvaluationServices::new(promotions_repo.clone(), coupons_repo.clone(), coupon_uses_repo.clone(), transaction_repo.clone(), message_sender.clone());
        let demographic = DemographyServices::new();
        let coupons = CouponServices::new(promotions_repo.clone(), coupons_repo.clone());
        let promotions = PromotionService::new(promotions_repo.clone(), organizations.clone(), message_sender.clone());
        Services {
            evaluation,
            demographic,
            organizations_repo: organizations,
            promotions_repo,
            message_sender,
            promotions,
            coupons_repo,
            coupons,
            coupon_uses_repo,
            transaction_repo,
        }
    }
}