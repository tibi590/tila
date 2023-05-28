mod db;
mod schema;
mod models;

fn main() {
    let connection = &mut db::get_connection();

    let users = db::show_users(connection);

    for user in users {
        println!("{:?}", user);
    }
}
