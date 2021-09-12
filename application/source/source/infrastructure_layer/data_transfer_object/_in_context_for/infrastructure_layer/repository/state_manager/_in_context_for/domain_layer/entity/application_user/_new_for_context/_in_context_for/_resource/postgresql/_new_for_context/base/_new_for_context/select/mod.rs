use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;

#[derive(Queryable)]
pub struct Select {
    id: i64,
    email: String,
    nickname: String,
    password_hash: String,
    created_at: ChronoDateTime<Utc>
}

impl Select {
    pub fn into_inner(
        self
    ) -> (i64, String, String, String, ChronoDateTime<Utc>) {
        return (
            self.id,
            self.email,
            self.nickname,
            self.password_hash,
            self.created_at
        );
    }
}