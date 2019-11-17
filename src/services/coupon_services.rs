use crate::models::{PromotionRepository, CouponsRepository, NewCoupon, Promotion, PromotionType};
use crate::server::{ApiResult, ApiError};
use crate::services::{GenerateCouponsDto, CouponsDto};

pub struct CouponServices {
    promotions_repo: PromotionRepository,
    coupons_repo: CouponsRepository,
}

impl CouponServices {
    pub fn new(promotions_repo: PromotionRepository, coupons_repo: CouponsRepository) -> Self {
        Self { promotions_repo, coupons_repo }
    }

    pub fn generate_coupons(&self, data: GenerateCouponsDto) -> ApiResult<Vec<CouponsDto>> {
        let promotion = self.promotions_repo.find(data.promotion_id)?;
        self.validate_promotion_is_coupons(&promotion)?;

        let coupon_codes: Vec<NewCoupon> = (0..data.quantity).into_iter()
            .map(|_| data.generate())
            .collect();

        Ok(self.coupons_repo
            .create_batch(&coupon_codes)?
            .into_iter()
            .map(CouponsDto::from)
            .collect()
        )
    }

    fn validate_promotion_is_coupons(&self, promotion: &Promotion) -> ApiResult<()> {
        if let PromotionType::Coupon = promotion.get_type() {
            Ok(())
        }
        else {
            Err(ApiError::from("Promotion is not coupon"))
        }
    }

    pub fn get_coupons(&self, promotion_id: i32) -> ApiResult<Vec<CouponsDto>> {
        Ok(self.coupons_repo
            .get_by_promotion(promotion_id)?
            .into_iter()
            .map(CouponsDto::from)
            .collect()
        )
    }
}
