use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::core::confirmed::Confirmed;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::entity::entity::application_user::core::password_hash::PasswordHash;
use crate::entity::entity::application_user::core::password::Password;
use crate::utility::entity::entity::application_user::password_encoder::PasswordEncoder;
use maybe_owned::MaybeOwned;

pub struct ApplicationUser<'b> {
    id: UuidV4<'b>,
    email: Email<'b>,                   // TODO где хранить сам токен и дату экспирации для подтверждении и логине  ( Редис?)
    nickname: Nickname<'b>,
    password_hash: PasswordHash<'b>,
    created_at: DateTime<'b>,           // TODO  Roles
    confirmed: Confirmed,
    password_encoder: PasswordEncoder
}

impl<'a, 'b: 'a> ApplicationUser<'b> {
    pub fn new_from_credentials(email: &'b String, nickname: &'b String, password: &'b String) -> Self {
        let password_encoder: PasswordEncoder = PasswordEncoder::new();

        return Self {
            id: UuidV4::new(),
            email: Email::new(MaybeOwned::Borrowed(email)),
            nickname: Nickname::new(MaybeOwned::Borrowed(nickname)),
            password_hash: PasswordHash::new(MaybeOwned::Owned(password_encoder.encode(password))),
            created_at: DateTime::new(),
            confirmed: Confirmed::new(false),
            password_encoder
        };
    }

    pub fn new_from_model(model: &'b Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(model.get_id()),
            email: Email::new(MaybeOwned::Borrowed(model.get_emal())),
            nickname: Nickname::new(MaybeOwned::Borrowed(model.get_nickname())),
            password_hash: PasswordHash::new(MaybeOwned::Borrowed(model.get_password_hash())),
            created_at: DateTime::new_from_date_time(MaybeOwned::Borrowed(model.get_created_at())),
            confirmed: Confirmed::new(model.get_confirmed()),
            password_encoder: PasswordEncoder::new()
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

    pub fn set_password(&'a mut self, password: Password<'b>) -> &'a mut Self {
        self.password_hash = PasswordHash::new(MaybeOwned::Owned(self.password_encoder.encode(password.get_value())));

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