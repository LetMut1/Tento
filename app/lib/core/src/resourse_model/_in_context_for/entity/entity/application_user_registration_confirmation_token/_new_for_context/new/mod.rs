use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::application_user_registration_confirmation_token;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "application_user_registration_confirmation_token"]
pub struct New<'outer> {
    id: &'outer Uuid,
    application_user_id: &'outer Uuid,
    value: &'outer String,
    expired_at: &'outer ChronoDateTime<Utc>
}

impl<'outer> New<'outer> {
    pub fn new(entity: &'outer ApplicationUserRegistrationConfirmationToken<'outer>) -> Self {
        return Self {
            id: entity.get_id().get_value(),
            application_user_id: entity.get_application_user_id().get_value(),
            value: entity.get_value().get_value(),
            expired_at: entity.get_expired_at().get_value()
        };
    }
}