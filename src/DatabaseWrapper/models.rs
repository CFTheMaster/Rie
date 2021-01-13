#[derive(Queryable)]
pub struct Tokens {
    pub id: i32,
    pub token_type: String,
    pub token: String
}

#[derive(Queryable)]
pub struct Blacklist {
    pub id: i32
}