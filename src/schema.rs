table! {
    appkeys (promotion_id, token) {
        promotion_id -> Int4,
        token -> Varchar,
        organization_id -> Varchar,
    }
}

table! {
    coupons (id, promotion_id) {
        id -> Int4,
        coupon_code -> Varchar,
        promotion_id -> Int4,
        expiration -> Timestamptz,
        max_uses -> Int4,
    }
}

table! {
    coupon_uses (coupon_id, promotion_id, external_user) {
        coupon_id -> Int4,
        promotion_id -> Int4,
        external_user -> Int4,
        uses -> Int4,
    }
}

table! {
    organizations (id) {
        id -> Varchar,
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
        organization_id -> Varchar,
        expiration -> Timestamptz,
    }
}

table! {
    transactions (id) {
        id -> Int4,
    }
}

joinable!(appkeys -> organizations (organization_id));
joinable!(appkeys -> promotions (promotion_id));
joinable!(coupons -> promotions (promotion_id));
joinable!(promotions -> organizations (organization_id));

allow_tables_to_appear_in_same_query!(
    appkeys,
    coupons,
    coupon_uses,
    organizations,
    promotions,
    transactions,
);
