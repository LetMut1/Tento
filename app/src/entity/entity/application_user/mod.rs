pub mod core;

use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::entity::core::uuid_v4::UuidV4;
use maybe_owned::MaybeOwned;
use self::core::email::Email;
use self::core::nickname::Nickname;
use self::core::password_hash::PasswordHash;

pub struct ApplicationUser<'b> {
    id: UuidV4<'b>,
    email: Email<'b>,
    nickname: Nickname<'b>,
    password_hash: PasswordHash<'b>
}

impl<'a, 'b: 'a> ApplicationUser<'b> {
    pub fn new_from_credentials(email: String, nickname: String, password: String) -> Self {     
        return Self {
            id: UuidV4::new(),
            email: Email::new(MaybeOwned::Owned(email)),
            nickname: Nickname::new(MaybeOwned::Owned(nickname)),
            password_hash: PasswordHash::new(MaybeOwned::Owned(password))
        };
    }

    pub fn new_from_model(model: &'b Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(model.get_id()),
            email: Email::new(MaybeOwned::Borrowed(model.get_emal())),
            nickname: Nickname::new(MaybeOwned::Borrowed(model.get_nickname())),
            password_hash: PasswordHash::new(MaybeOwned::Borrowed(model.get_password_hash()))
        };
    }

    pub fn set_email(&'a mut self, email: Email<'b>) -> &'a mut Self {
        self.email = email;

        return self;
    }

    pub fn set_nickname(&'a mut self, nickname: Nickname<'b>) -> &'a mut Self {
        self.nickname = nickname;

        return self;
    }

    pub fn set_password_hash(&'a mut self, password: String) -> &'a mut Self {
        self.password_hash = PasswordHash::new(MaybeOwned::Owned(password));

        return self;
    }

    pub fn get_id(&'a self) -> &'a UuidV4<'b> {
        return &self.id;
    }

    pub fn get_emal(&'a self) -> &'a Email<'b> {
        return &self.email;
    }

    pub fn get_nickname(&'a self) -> &'a Nickname<'b> {
        return &self.nickname;
    }

    pub fn get_passord_hash(&'a self) -> &'a PasswordHash<'b> {
        return &self.password_hash;
    }
}