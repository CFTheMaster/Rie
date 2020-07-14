use diesel::prelude::*;

use crate::DatabaseWrapper::schema;

#[derive(Queryable)]
pub struct Tokens {
    pub token_type: String,
    pub token: String
}

#[derive(Queryable)]
pub struct Blacklist {
    pub id: i32
}

pub fn establish_connection(&str: url) -> PgConnection {
    PgConnection::establish(url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn fetchToken(&str: url) -> String {
    use schema::tokens::dsl::*;

    let connection = establish_connection(url);
    let results = tokens
        .limit(1)
        .load::<Tokens>(&connection)
        .expect("Error retrieving discord token");

    return tokens.token;
}