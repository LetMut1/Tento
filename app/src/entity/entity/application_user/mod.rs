pub mod core;

use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use maybe_owned::MaybeOwned;
use self::core::confirmed::Confirmed;
use self::core::email::Email;
use self::core::nickname::Nickname;
use self::core::password_hash::PasswordHash;

pub struct ApplicationUser<'b> {
    id: UuidV4<'b>,
    email: Email<'b>,                   // TODO где хранить сам токен и дату экспирации для подтверждении и логине  ( Редис?)
    nickname: Nickname<'b>,
    password_hash: PasswordHash<'b>,
    created_at: DateTime<'b>,           // TODO  Roles
    confirmed: Confirmed
}

impl<'a, 'b: 'a> ApplicationUser<'b> {
    pub fn new_from_credentials(email: &'b String, nickname: &'b String, password: &'b String) -> Self {     
        return Self {
            id: UuidV4::new(),
            email: Email::new(MaybeOwned::Borrowed(email)),
            nickname: Nickname::new(MaybeOwned::Borrowed(nickname)),
            password_hash: PasswordHash::new(MaybeOwned::Borrowed(password)),
            created_at: DateTime::new(),
            confirmed: Confirmed::new(false)
        };
    }

    pub fn new_from_model(model: &'b Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(model.get_id()),
            email: Email::new(MaybeOwned::Borrowed(model.get_emal())),
            nickname: Nickname::new(MaybeOwned::Borrowed(model.get_nickname())),
            password_hash: PasswordHash::new(MaybeOwned::Borrowed(model.get_password_hash())),
            created_at: DateTime::new_from_date_time(MaybeOwned::Borrowed(model.get_created_at())),
            confirmed: Confirmed::new(model.get_confirmed())
        };
    }

    pub fn is_confirmed(&'a self) -> bool {
        return (&self.confirmed).get_value();
    }

    pub fn set_email(&'a mut self, email: Email<'b>) -> &'a mut Self {
        self.email = email;

        return self;
    }

    pub fn set_nickname(&'a mut self, nickname: Nickname<'b>) -> &'a mut Self {
        self.nickname = nickname;

        return self;
    }

    pub fn set_password_hash(&'a mut self, password_hash: PasswordHash<'b>) -> &'a mut Self {
        self.password_hash = password_hash;

        return self;
    }

    pub fn get_id(&'a self) -> &'a UuidV4<'b> {
        return &self.id;
    }

    pub fn get_email(&'a self) -> &'a Email<'b> {
        return &self.email;
    }

    pub fn get_nickname(&'a self) -> &'a Nickname<'b> {
        return &self.nickname;
    }

    pub fn get_passord_hash(&'a self) -> &'a PasswordHash<'b> {
        return &self.password_hash;
    }

    pub fn get_created_at(&'a self) -> &'a DateTime<'b> {
        return &self.created_at;
    }

    pub fn get_confirmed(&'a self) -> &'a Confirmed {
        return &self.confirmed;
    }
}