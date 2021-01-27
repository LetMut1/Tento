use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use crate::diesel_component::schema::public::json_refresh_web_token;
use crate::entity::entity::json_web_token::json_refresh_web_token::JsonRefreshWebToken;
use diesel::Insertable;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "json_refresh_web_token"]
pub struct New<'a> {
    id: &'a Uuid,
    user_id: &'a Uuid,
    device_id: &'a String,
    value_hash: &'a String,
    created_at: &'a ChronoDateTime<Utc>,
    expired_at: &'a ChronoDateTime<Utc>
}

impl<'a> New<'a> {
    pub fn from_entity(entity: &'a JsonRefreshWebToken) -> Self {
        return Self {
            id: entity.get_id().get_value(),
            user_id: entity.get_user_id().get_value(),
            device_id: entity.get_device_id().get_value(),
            value_hash: entity.get_value_hash().get_value(),
            created_at: entity.get_created_at().get_value(),
            expired_at: entity.get_created_at().get_value()
        };
    }
}

#[derive(Queryable)]
pub struct Existing {
    id: Uuid,
    user_id: Uuid,
    device_id: String,
    value_hash: String,
    created_at: ChronoDateTime<Utc>,
    expired_at: ChronoDateTime<Utc>
}

impl<'a> Existing {
    pub fn get_id(&'a self) -> &'a Uuid {
        return &self.id;
    }

    pub fn get_user_id(&'a self) -> &'a Uuid {
        return &self.user_id;
    }

    pub fn get_device_id(&'a self) -> &'a String {
        return &self.device_id;
    }

    pub fn get_value_hash(&'a self) -> &'a String {
        return &self.value_hash;
    }

    pub fn get_created_at(&'a self) -> &'a ChronoDateTime<Utc> {
        return &self.created_at;
    }

    pub fn get_expired_at(&'a self) -> &'a ChronoDateTime<Utc> {
        return &self.expired_at;
    }
}