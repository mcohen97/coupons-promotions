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
    }
}
