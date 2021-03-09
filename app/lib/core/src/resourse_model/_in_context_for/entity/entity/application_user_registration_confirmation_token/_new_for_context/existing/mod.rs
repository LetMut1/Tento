use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Existing {
    id: Uuid,
    application_user_id: Uuid,
    value: String,
    created_at: ChronoDateTime<Utc>,
}

impl<'this> Existing {
    pub fn get_id(&'this self) -> &'this Uuid {
        return &self.id;
    }

    pub fn get_application_user_id(&'this self) -> &'this Uuid {
        return &self.application_user_id;
    }

    pub fn get_value(&'this self) -> &'this String {
        return &self.value;
    }

    pub fn get_created_at(&'this self) -> &'this ChronoDateTime<Utc> {
        return &self.created_at;
    }
}