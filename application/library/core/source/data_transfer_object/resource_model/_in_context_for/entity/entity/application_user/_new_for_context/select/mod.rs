use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Select {
    pub id: Uuid,
    pub email: String,
    pub nickname: String,
    pub password_hash: String,
    pub created_at: ChronoDateTime<Utc>
}