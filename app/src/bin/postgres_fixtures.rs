extern crate self_lib;
use self_lib::diesel_component::schema::public::app_user;
use self_lib::entity::entity::app_user::AppUser;

extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;                   // TODO Write normal (in one Connection)

fn main() -> () {
    for i in 1..100 {
        diesel::insert_into(app_user::table)
        .values(& AppUser::new(i.to_string(), self_lib::entity::entity::app_user::core::id::Id::new().get_value(), String::from("svdf")))
        .execute(& PgConnection::establish("postgres://user:password@postgresql/memis").expect(&format!("Error connecting")))
        .expect("Error saving new post");
    }
    
    return ();
}