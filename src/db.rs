use diesel::sqlite::SqliteConnection;
use diesel::{prelude::*, insert_into, delete};
use dotenvy::dotenv;
use std::env;
use crate::functions::input_to_var;
use crate::models::*;
use crate::schema::users::dsl::*;

pub fn get_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL erro.");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn get_users(connection: &mut SqliteConnection) -> Vec<User>{
    use crate::schema::users::dsl::*;

    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users.");

    results
}

pub fn write_user(connection: &mut SqliteConnection, user: &User) -> QueryResult<usize> {
    insert_into(users)
        .values((username.eq(&user.username), password.eq(&user.password)))
        .execute(connection)
}

pub fn delete_user(connection: &mut SqliteConnection) {
    println!("\nDELETION");

    let temp_username = input_to_var("\nUsername: ");
    let pattern = format!("%{}%", temp_username);

    if !temp_username.is_empty() {
        delete(users.filter(username.like(pattern)))
            .execute(connection)
            .expect("Error deleting user");
    }
}
