use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::json_refresh_web_token;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "json_refresh_web_token"]
pub struct New<'outer> {
    id: &'outer Uuid,
    application_user_id: &'outer Uuid,
    device_id: &'outer Uuid,
    value: &'outer str,
    created_at: &'outer ChronoDateTime<Utc>,
    expired_at: &'outer ChronoDateTime<Utc>
}

impl<'outer> New<'outer> {
    pub fn new(json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>) -> Self {
        return Self {
            id: json_refresh_web_token.get_id().get_value(),
            application_user_id: json_refresh_web_token.get_application_user_id().get_value(),
            device_id: json_refresh_web_token.get_device_id().get_value(),
            value: json_refresh_web_token.get_value().get_value(),
            created_at: json_refresh_web_token.get_created_at().get_value(),
            expired_at: json_refresh_web_token.get_expired_at().get_value()
        };
    }
}