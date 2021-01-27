extern crate self_lib;
use self_lib::diesel_component::schema::public::application_user;
use self_lib::entity::entity::application_user::ApplicationUser;

extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
                                // TODO refactor with any methods
fn main() -> () {
    // let mut application_user_registry: Vec<ApplicationUser> = vec![];
    
    // for i in 1..1000 {
    //     application_user_registry.push(
    //         ApplicationUser::new(i.to_string(), i.to_string())
    //     )
    // }

    // diesel::insert_into(application_user::table)
    // .values(&application_user_registry)
    // .execute(&PgConnection::establish("postgres://root:password@postgresql/mem_is").expect(&format!("Error connecting")))
    // .expect("Error saving new post");
    
    return ();
}