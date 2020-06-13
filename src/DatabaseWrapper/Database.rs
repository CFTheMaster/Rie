use diesel::prelude::*;

#[derive(Queryable)]
pub struct tokens {
    pub discordToken: String,
}

#[derive(Queryable)]
pub struct users {
    pub users: i32,
    pub blacklisted: i8,
}

pub fn establish_connection -> PgConnection {
    let database_url = "127.0.0.1:5432"
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn getPosts() {
    use self::DatabaseWrapper::schema::discordToken::dsl::*;

    let connection = establish_connection();
    let results = discordToken
        .limit(1)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", discordToken.discordToken);
    }
}