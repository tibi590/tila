use crate::db::*;
use crate::models::*;
use diesel::SqliteConnection;
use std::io::{self, Write};

pub fn login(connection: &mut SqliteConnection) -> bool {
    println!("LOGIN\n");

    let username = input_to_var("Username: ");
    let password = input_to_var("Password: ");

    let users = get_users(connection);

    for user in users {
        println!("{:?}", user);
        println!("{}, {}", username.trim(), password.trim());
        println!("-----------------------------");
        if user.username == username && user.password == password {
            return true;
        }
    }
    return false;
}

pub fn register(connection: &mut SqliteConnection) -> User {
    println!("REGISTRATION\n");
    
    loop {
        let temp_user = User {
            username: input_to_var("Username: "),
            password: input_to_var("Password: "),
        };

        if let Err(_) = write_user(connection, &temp_user) {
            println!("Username already taken. Try again.");
        } else {
            return temp_user;
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

    var.trim_end().to_string()
}
