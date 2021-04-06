use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Existing {
    pub json_access_web_token_id: Uuid,
    pub expired_at: ChronoDateTime<Utc>
}