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
    pre_confirmed_application_user_id: &'outer Uuid,
    application_user_email: &'outer str,
    value: &'outer str,
    expired_at: &'outer ChronoDateTime<Utc>
}

impl<'outer> New<'outer> {
    pub fn new(application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>) -> Self {
        return Self {
            id: application_user_registration_confirmation_token.get_id().get_value(),
            pre_confirmed_application_user_id: application_user_registration_confirmation_token.get_pre_confirmed_application_user_id().get_value(),
            application_user_email: application_user_registration_confirmation_token.get_application_user_email().get_value(),
            value: application_user_registration_confirmation_token.get_value().get_value(),
            expired_at: application_user_registration_confirmation_token.get_expired_at().get_value()
        };
    }
}