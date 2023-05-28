use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::*;

pub fn get_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL erro.");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn show_users(connection: &mut SqliteConnection) -> Vec<User>{
    use crate::schema::users::dsl::*;

    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users.");

    results
}
