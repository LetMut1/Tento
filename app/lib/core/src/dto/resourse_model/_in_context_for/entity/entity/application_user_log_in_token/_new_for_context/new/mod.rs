use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::application_user_log_in_token;
use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "application_user_log_in_token"]
pub struct New<'outer> {
    id: &'outer Uuid,
    application_user_id: &'outer Uuid,
    device_id: &'outer Uuid,
    value: &'outer str,
    expired_at: &'outer ChronoDateTime<Utc>
}

impl<'outer> New<'outer> {
    pub fn new(application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>) -> Self {
        return Self {
            id: application_user_log_in_token.get_id().get_value(),
            application_user_id: application_user_log_in_token.get_application_user_id().get_value(),
            device_id: application_user_log_in_token.get_device_id().get_value(),
            value: application_user_log_in_token.get_value().get_value(),
            expired_at: application_user_log_in_token.get_expired_at().get_value()
        };
    }
}