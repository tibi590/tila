use diesel::prelude::*;

use crate::db::get_users;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn help(&self) {
        print!("-help          |Show prompts
-list-profiles |Lists all profiles
-exit          |Exit
");
    }

    pub fn list_profiles(&self, connection: &mut SqliteConnection) {
        let users = get_users(connection);
        for user in users {
            println!("{} | {}", user.username, user.password);
        }
    }
}
