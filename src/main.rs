mod db;
mod schema;
mod models;
mod functions;
mod pages;

use db::get_connection;
use pages::*;
use functions::input_to_var;

fn main() {
    let connection = &mut get_connection();

    loop {
        let prompt = input_to_var("\nRegister | Login | Exit (r | l | e): ");

        match prompt.to_lowercase().as_str() {
            "r" => register_page(connection),
            "l" => login_page(connection),
            "e" => break,
            _ => println!("Invalid input"),
        }
    }
}
