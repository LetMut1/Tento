use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Select {
    id: i64,
    application_user_email: String,
    created_at: ChronoDateTime<Utc>
}

impl Select {
    pub fn into_inner(
        self
    ) -> (i64, String, ChronoDateTime<Utc>) {
        return (
            self.id,
            self.application_user_email,
            self.created_at
        );
    }
}