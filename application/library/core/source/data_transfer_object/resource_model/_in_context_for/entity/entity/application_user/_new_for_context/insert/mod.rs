use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::application_user;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "application_user"]
pub struct Insert<'outer_a> {                    // TODO описать id Аттрибутами
    id: &'outer_a Uuid,
    email: &'outer_a str,
    nickname: &'outer_a str,
    password_hash: &'outer_a str,
    created_at: &'outer_a ChronoDateTime<Utc>
}

impl<'outer_a> Insert<'outer_a> {
    pub fn new(application_user: &'outer_a ApplicationUser<'outer_a>) -> Self {
        return Self {
            id: application_user.get_id().get_value().get_value(),
            email: application_user.get_email().get_value(),
            nickname: application_user.get_nickname().get_value(),
            password_hash: application_user.get_passord_hash().get_value(),
            created_at: application_user.get_created_at().get_value().get_value()
        };
    }
}
