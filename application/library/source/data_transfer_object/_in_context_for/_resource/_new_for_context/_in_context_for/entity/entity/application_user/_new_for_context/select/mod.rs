use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Select {
    id: Uuid,
    email: String,
    nickname: String,
    password_hash: String,
    created_at: ChronoDateTime<Utc>
}

impl Select {
    pub fn into_inner(self) -> (Uuid, String, String, String, ChronoDateTime<Utc>) {
        return (
            self.id,
            self.email,
            self.nickname,
            self.password_hash,
            self.created_at
        );
    }
}