table! {
    coupons (coupon_code) {
        coupon_code -> Varchar,
        promotion_id -> Int4,
    }
}

table! {
    coupon_uses (coupon_code, external_user) {
        coupon_code -> Varchar,
        external_user -> Int4,
    }
}

table! {
    organizations (id) {
        id -> Int4,
    }
}

table! {
    promotions (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
        active -> Bool,
        return_type -> Varchar,
        return_value -> Float8,
        #[sql_name = "type"]
        type_ -> Varchar,
        organization_id -> Int4,
        expiration -> Timestamptz,
    }
}

joinable!(coupon_uses -> coupons (coupon_code));
joinable!(coupons -> promotions (promotion_id));
joinable!(promotions -> organizations (organization_id));

allow_tables_to_appear_in_same_query!(
    coupons,
    coupon_uses,
    organizations,
    promotions,
);
