pub mod core;

use crate::diesel_component::schema::public::app_user;
use diesel::Insertable;
use self::core::email::Email;
use self::core::id::Id;
use self::core::jwt_id::JwtId;
use self::core::nickname::Nickname;

#[derive(Insertable)]
#[table_name = "app_user"]
pub struct AppUser {
    email: Email,
    id: Id,
    jwt_id: JwtId,
    nickname: Nickname
}

impl<'a> AppUser {
    pub fn new(email: String, jwt_id: String, nickname: String) -> Self {
        return Self {email: Email::new(email), id: Id::new(), jwt_id: JwtId::new(jwt_id), nickname: Nickname::new(nickname)};
    }

    pub fn set_email(&'a mut self, email: Email) -> &'a mut Self {
        self.email = email;

        return self;
    }

    pub fn set_id(&'a mut self, id: Id) -> &'a mut Self {
        self.id = id;

        return self;
    }

    pub fn set_jwt_id(&'a mut self, jwt_id: JwtId) -> &'a mut Self {
        self.jwt_id = jwt_id;

        return self;
    }

    pub fn set_nickname(&'a mut self, nickname: Nickname) -> &'a mut Self {
        self.nickname = nickname;

        return self;
    }

    pub fn get_emal(&'a mut self) -> &'a mut Email {
        return &mut self.email;
    }

    pub fn get_id(&'a mut self) -> &'a mut Id {
        return &mut self.id;
    }

    pub fn get_jwt_id(&'a mut self) -> &'a mut JwtId {
        return &mut self.jwt_id;
    }

    pub fn get_nickname(&'a mut self) -> &'a mut Nickname {
        return &mut self.nickname;
    }
}