use crate::db::*;
use crate::models::*;
use diesel::SqliteConnection;
use std::io::{self, Write};
use rpassword::prompt_password;

pub fn login(connection: &mut SqliteConnection) -> bool {
    let username = input_to_var("\nUsername: ");
    let password = prompt_password("Password: ").unwrap();

    let users = get_users(connection);

    for user in users {
        if user.username == username && user.password == password {
            return true;
        }
    }
    return false;
}

pub fn register(connection: &mut SqliteConnection) -> bool {
    let user = User {
        username: input_to_var("\nUsername: "),
        password: prompt_password("Password: ").unwrap(),
    };

    if !user.username.is_empty() && !user.password.is_empty() {
        if let Err(_) = write_user(connection, &user) {
            println!("Username already taken. Try again.");
        } else {
            return true;
        }
    } else {
        println!("Invalid username. Try again.");
    }

    return false;
}

pub fn input_to_var(message: &str) -> String {
    let mut var = String::new();
    print!("{}", message);
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut var)
        .expect("Error reading input.");

    var.trim().to_string()
}
