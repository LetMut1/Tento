use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Existing {
    pub id: Uuid,
    pub pre_confirmed_application_user_id: Uuid,
    pub application_user_email: String,
    pub value: String,
    pub expired_at: ChronoDateTime<Utc>,
}