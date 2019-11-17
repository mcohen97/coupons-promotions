use crate::models::{PromotionRepository, CouponsRepository, NewCoupon};
use crate::server::ApiResult;
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
        let _ = self.promotions_repo.find(data.promotion_id)?;

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

    pub fn get_coupons(&self, promotion_id: i32) -> ApiResult<Vec<CouponsDto>> {
        Ok(self.coupons_repo
            .get_by_promotion(promotion_id)?
            .into_iter()
            .map(CouponsDto::from)
            .collect()
        )
    }
}
