use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Existing {
    pub id: Uuid,
    pub application_user_id: Uuid,
    pub application_user_log_in_token_device_id: Uuid,
    pub value: String,
    pub created_at: ChronoDateTime<Utc>,
    pub expired_at: ChronoDateTime<Utc>
}