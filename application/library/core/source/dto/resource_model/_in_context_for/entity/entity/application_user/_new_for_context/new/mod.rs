use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::application_user;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "application_user"]
pub struct New<'outer> {                    // TODO описать id Аттрибутами
    id: &'outer Uuid,
    email: &'outer str,
    nickname: &'outer str,
    password_hash: &'outer str,
    created_at: &'outer ChronoDateTime<Utc>
}

impl<'outer> New<'outer> {
    pub fn new(application_user: &'outer ApplicationUser<'outer>) -> Self {
        return Self {
            id: application_user.get_id().get_value(),
            email: application_user.get_email().get_value(),
            nickname: application_user.get_nickname().get_value(),
            password_hash: application_user.get_passord_hash().get_value(),
            created_at: application_user.get_created_at().get_value()
        };
    }
}
