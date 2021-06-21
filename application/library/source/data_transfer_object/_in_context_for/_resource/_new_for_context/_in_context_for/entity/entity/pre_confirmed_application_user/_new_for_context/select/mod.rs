use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Select {
    id: Uuid,
    application_user_email: String,
    created_at: ChronoDateTime<Utc>
}

impl Select {
    pub fn into_inner(self) -> (Uuid, String, ChronoDateTime<Utc>) {
        return (
            self.id,
            self.application_user_email,
            self.created_at
        );
    }
}