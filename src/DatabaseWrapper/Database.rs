use crate::DatabaseWrapper::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::DatabaseWrapper::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in env");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn getToken() -> String {
    let pls: String = "please end your life, kthx".to_owned();
    use schema::tokens::dsl::*;

    let connection = establish_connection();
    
    let results = tokens
        .filter(id.eq(1))
        .load::<Tokens>(&connection)
        .expect("Error getting tokens");

    println!("is it working? {}", results.len());

    for post in results {
        let niceTokenBeLike: Option<String> = Some(post.token);
        
        let niceToken = niceTokenBeLike.as_ref().unwrap_or(&pls);

        return niceToken.to_string();
    };

    let none: Option<String> = Some("1234".to_string());
    let fuckYou = none.as_ref().unwrap_or(&pls);
    
    return fuckYou.to_string();
}

