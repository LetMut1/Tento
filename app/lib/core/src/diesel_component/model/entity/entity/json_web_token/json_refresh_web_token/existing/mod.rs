use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Existing {
    id: Uuid,
    user_id: Uuid,
    device_id: String,
    value: String,
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
        return &self.value;
    }

    pub fn get_created_at(&'a self) -> &'a ChronoDateTime<Utc> {
        return &self.created_at;
    }

    pub fn get_expired_at(&'a self) -> &'a ChronoDateTime<Utc> {
        return &self.expired_at;
    }
}