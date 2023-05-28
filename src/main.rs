mod db;
mod schema;
mod models;
mod functions;

use db::*;
use crate::functions::{login, register};

fn main() {
    let connection = &mut get_connection();

    println!("{}", login(connection));

    register(connection);

    for user in get_users(connection) {
        println!("{:?}", user);
    }

    delete_user(connection);

    for user in get_users(connection) {
        println!("{:?}", user);
    }
}
