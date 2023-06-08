use diesel::prelude::*;
use crate::models::User;
use crate::functions::{input_to_var, login, register};
use rpassword::prompt_password;

pub fn register_page(connection: &mut SqliteConnection) {
    println!("\nREGISTRATION");
    loop {
        if register(connection) {
            println!("Succesful registration");    
            break;
        }
    }
}

pub fn login_page(connection: &mut SqliteConnection) {
    println!("\nLOGIN");

    for _ in 0..2 {
        let username = input_to_var("\nUsername: ");
        let password = prompt_password("Password: ").unwrap();

        if login(connection, &username, &password) {
            let user = User{username, password};
            menu_page(connection, user);
            break;
        }
    }
}

pub fn menu_page(connection: &mut SqliteConnection, user: User) {
    println!("\nMENU\n");

    loop {
        let prompt = input_to_var("->");

        match prompt.as_str() {
            "help" | "?" => user.help(),
            "list-profiles" => user.list_profiles(connection),
            "exit" => break,
            _ => println!("Invalid input"),
        };
    }
}
