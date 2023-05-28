use crate::db::*;
use crate::models::*;
use diesel::SqliteConnection;
use std::io::{self, Write};

pub fn login(connection: &mut SqliteConnection) -> bool {
    println!("\nLOGIN");

    let username = input_to_var("\nUsername: ");
    let password = input_to_var("Password: ");

    let users = get_users(connection);

    for user in users {
        if user.username == username && user.password == password {
            return true;
        }
    }
    return false;
}

pub fn register(connection: &mut SqliteConnection) -> User {
    println!("\nREGISTRATION");
    
    loop {
        let temp_user = User {
            username: input_to_var("\nUsername: "),
            password: input_to_var("Password: "),
        };

        if !(temp_user.username.is_empty() && temp_user.password.is_empty()) {
            if let Err(_) = write_user(connection, &temp_user) {
                println!("Username already taken. Try again.");
            } else {
                return temp_user;
            }
        } else {
            println!("Invalid username. Try again.");
        }
    }
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
