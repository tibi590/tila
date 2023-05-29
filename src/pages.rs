use diesel::prelude::*;
use crate::functions::{input_to_var, login, register};

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
        if login(connection) {
            menu_page(connection);
            break;
        }
    }
}

pub fn menu_page(_connection: &mut SqliteConnection) {
    println!("\nMENU\n");

    loop {
        let prompt = input_to_var("->");

        match prompt.as_str() {
            "exit" => break,
            _ => println!("Invalid input"),
        };
    }
}
