
use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::application_user;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "application_user"]
pub struct New<'b> {                    // TODO описать id Аттрибутами
    id: &'b Uuid,
    email: &'b String,
    nickname: &'b String,
    password_hash: &'b String,
    created_at: &'b ChronoDateTime<Utc>,
    confirmed: bool
}

impl<'b> New<'b> {
    pub fn new_from_entity(application_user: &'b ApplicationUser<'b>) -> Self {
        return Self {
            id: application_user.get_id().get_value(),
            email: application_user.get_email().get_value(),
            nickname: application_user.get_nickname().get_value(),
            password_hash: application_user.get_passord_hash().get_value(),
            created_at: application_user.get_created_at().get_value(),
            confirmed: application_user.get_confirmed().get_value()
        };
    }
}
