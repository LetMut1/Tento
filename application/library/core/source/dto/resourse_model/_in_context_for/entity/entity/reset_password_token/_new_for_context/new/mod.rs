use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::reset_password_token;
use crate::entity::entity::reset_password_token::reset_password_token::ResetPasswordToken;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "reset_password_token"]
pub struct New<'outer> {
    id: &'outer Uuid,
    application_user_email: &'outer str,
    value: &'outer str,
    expired_at: &'outer ChronoDateTime<Utc>
}

impl<'outer> New<'outer> {
    pub fn new(reset_password_token: &'outer ResetPasswordToken<'outer>) -> Self {
        return Self {
            id: reset_password_token.get_id().get_value(),
            application_user_email: reset_password_token.get_application_user_email().get_value(),
            value: reset_password_token.get_value().get_value(),
            expired_at: reset_password_token.get_expired_at().get_value()
        };
    }
}