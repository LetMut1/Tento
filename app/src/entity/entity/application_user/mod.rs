pub mod core;

use crate::diesel_component::schema::public::application_user;
use crate::entity::core::uuid_v4::UuidV4;
use diesel::Insertable;
use diesel::Queryable;
use self::core::email::Email;
use self::core::nickname::Nickname;

#[derive(Insertable, Queryable)]
#[table_name = "application_user"]
pub struct ApplicationUser{
    id: UuidV4,
    email: Email,
    nickname: Nickname
}

impl<'a> ApplicationUser {
    pub fn new(email: String, nickname: String) -> Self {
        return Self {id: UuidV4::new(), email: Email::new(email), nickname: Nickname::new(nickname)};
    }

    pub fn set_email(&'a mut self, email: Email) -> &'a mut Self {
        self.email = email;

        return self;
    }

    pub fn set_nickname(&'a mut self, nickname: Nickname) -> &'a mut Self {
        self.nickname = nickname;

        return self;
    }

    pub fn get_id(&'a mut self) -> &'a mut UuidV4 {
        return &mut self.id;
    }

    pub fn get_emal(&'a mut self) -> &'a mut Email {
        return &mut self.email;
    }

    pub fn get_nickname(&'a mut self) -> &'a mut Nickname {
        return &mut self.nickname;
    }
}