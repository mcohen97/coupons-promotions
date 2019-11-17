-- Your SQL goes here

ALTER TABLE coupon_uses ADD uses INTEGER NOT NULL DEFAULT 0;
ALTER TABLE coupons ADD max_uses INTEGER NOT NULL;