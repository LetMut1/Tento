use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Existing {
    id: Uuid,
    email: String,
    nickname: String,
    password_hash: String
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
}