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

impl<'this> Existing{
    pub fn get_id(&'this self) -> &'this Uuid {
        return &self.id;
    }

    pub fn get_emal(&'this self) -> &'this String {
        return &self.email;
    }

    pub fn get_nickname(&'this self) -> &'this String {
        return &self.nickname;
    }

    pub fn get_password_hash(&'this self) -> &'this String {
        return &self.password_hash;
    }

    pub fn get_created_at(&'this self) -> &'this ChronoDateTime<Utc> {
        return &self.created_at;
    }

    pub fn get_confirmed(&'this self) -> bool {
        return self.confirmed;
    }
}