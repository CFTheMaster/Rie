diesel::table! {
    tokens (type) {
        token_type -> String,
        token -> String,
    }

    blacklist (id) {
        id -> Int4,
    }
}