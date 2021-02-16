use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::json_refresh_web_token;
use crate::entity::entity::json_web_token::json_refresh_web_token::JsonRefreshWebToken;
use diesel::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "json_refresh_web_token"]
pub struct New<'b> {
    id: &'b Uuid,
    user_id: &'b Uuid,
    device_id: &'b String,
    value: &'b String,
    created_at: &'b ChronoDateTime<Utc>,
    expired_at: &'b ChronoDateTime<Utc>
}

impl<'a, 'b: 'a> New<'b> {
    pub fn from_entity(entity: &'b JsonRefreshWebToken<'a, 'b>) -> Self {
        return Self {
            id: entity.get_id().get_value(),
            user_id: entity.get_user_id().get_value(),
            device_id: entity.get_device_id().get_value(),
            value: entity.get_value().get_value(),
            created_at: entity.get_created_at().get_value(),
            expired_at: entity.get_created_at().get_value()
        };
    }
}