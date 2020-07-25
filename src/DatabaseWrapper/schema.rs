table! {
    blacklist (id) {
        id -> Int4,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        token_type -> Varchar,
        token -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    blacklist,
    tokens,
);
