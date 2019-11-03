table! {
    promotions (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
        active -> Bool,
        return_type -> Int4,
        return_value -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        organization_id -> Int4,
        invocations -> Int4,
        negative_responses -> Int4,
        average_response_time -> Float8,
        total_spent -> Float8,
    }
}
