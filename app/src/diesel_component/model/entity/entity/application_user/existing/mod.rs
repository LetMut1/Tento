use chrono::DateTime as ChronoDateTime;
use chrono::offset::Utc;
use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Existing {
    id: Uuid,
    email: String,
    nickname: String,
    password_hash: String,
    created_at: ChronoDateTime<Utc>,
    confirmed: bool
}

impl<'a> Existing{
    pub fn get_id(&'a self) -> &'a Uuid {
        return &self.id;
    }

    pub fn get_emal(&'a self) -> &'a String {
        return &self.email;
    }

    pub fn get_nickname(&'a self) -> &'a String {
        return &self.nickname;
    }

    pub fn get_password_hash(&'a self) -> &'a String {
        return &self.password_hash;
    }

    pub fn get_created_at(&'a self) -> &'a ChronoDateTime<Utc> {
        return &self.created_at;
    }

    pub fn get_confirmed(&'a self) -> bool {
        return self.confirmed;
    }
}