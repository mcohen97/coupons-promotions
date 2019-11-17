-- This file should undo anything in `up.sql`

ALTER TABLE coupon_uses DROP uses;
ALTER TABLE coupons ADD max_uses INTEGER NOT NULL;