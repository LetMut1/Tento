use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::pre_registered_application_user_registration_confirmation_token;
use crate::entity::entity::pre_registered_application_user_registration_confirmation_token::pre_registered_application_user_registration_confirmation_token::PreRegisteredApplicationUserRegistrationConfirmationToken;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "pre_registered_application_user_registration_confirmation_token"]
pub struct New<'outer> {
    id: &'outer Uuid,
    pre_confirmed_application_user_id: &'outer Uuid,
    value: &'outer str,
    expired_at: &'outer ChronoDateTime<Utc>
}

impl<'outer> New<'outer> {
    pub fn new(pre_registered_application_user_registration_confirmation_token: &'outer PreRegisteredApplicationUserRegistrationConfirmationToken<'outer>) -> Self {
        return Self {
            id: pre_registered_application_user_registration_confirmation_token.get_id().get_value(),
            pre_confirmed_application_user_id: pre_registered_application_user_registration_confirmation_token.get_pre_confirmed_application_user_id().get_value(),
            value: pre_registered_application_user_registration_confirmation_token.get_value().get_value(),
            expired_at: pre_registered_application_user_registration_confirmation_token.get_expired_at().get_value()
        };
    }
}