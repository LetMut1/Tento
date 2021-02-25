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
    user_id: &'outer Uuid,
    device_id: &'outer String,
    value: &'outer String,
    created_at: &'outer ChronoDateTime<Utc>,
    expired_at: &'outer ChronoDateTime<Utc>
}

impl<'outer> New<'outer> {
    pub fn new_from_entity(entity: &'outer JsonRefreshWebToken<'outer, 'outer>) -> Self {
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