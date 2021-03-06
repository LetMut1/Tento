use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::core::confirmed::Confirmed;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::entity::entity::application_user::core::password_hash::PasswordHash;
use crate::entity::entity::application_user::core::password::Password;
use crate::utility::entity::entity::application_user::core::password::password_encoder::PasswordEncoder;
use maybe_owned::MaybeOwned;

pub struct ApplicationUser<'outer> {
    id: UuidV4<'outer>,
    email: Email<'outer>,
    nickname: Nickname<'outer>,
    password_hash: PasswordHash<'outer>,
    created_at: DateTime<'outer>,           // TODO  Roles
    confirmed: Confirmed
}

impl<'this, 'outer: 'this> ApplicationUser<'outer> {
    pub fn new(email: &'outer String, nickname: &'outer String, password: &'outer String) -> Self {
        return Self {
            id: UuidV4::new(),
            email: Email::new(MaybeOwned::Borrowed(email)),
            nickname: Nickname::new(MaybeOwned::Borrowed(nickname)),
            password_hash: PasswordHash::new(MaybeOwned::Owned(PasswordEncoder::encode(password))),
            created_at: DateTime::new(),
            confirmed: Confirmed::new(false)
        };
    }

    pub fn new_from_model(existing: &'outer Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.get_id()),
            email: Email::new(MaybeOwned::Borrowed(existing.get_emal())),
            nickname: Nickname::new(MaybeOwned::Borrowed(existing.get_nickname())),
            password_hash: PasswordHash::new(MaybeOwned::Borrowed(existing.get_password_hash())),
            created_at: DateTime::new_from_date_time(MaybeOwned::Borrowed(existing.get_created_at())),
            confirmed: Confirmed::new(existing.get_confirmed())
        };
    }

    pub fn is_confirmed(&'this self) -> bool {
        return (&self.confirmed).get_value();
    }

    pub fn set_email(&'this mut self, email: Email<'outer>) -> &'this mut Self {
        self.email = email;

        return self;
    }

    pub fn set_nickname(&'this mut self, nickname: Nickname<'outer>) -> &'this mut Self {
        self.nickname = nickname;

        return self;
    }

    pub fn set_password(&'this mut self, password: Password<'outer>) -> &'this mut Self {
        self.password_hash = PasswordHash::new(MaybeOwned::Owned(PasswordEncoder::encode(password.get_value())));

        return self;
    }

    pub fn get_id(&'this self) -> &'this UuidV4<'outer> {
        return &self.id;
    }

    pub fn get_email(&'this self) -> &'this Email<'outer> {
        return &self.email;
    }

    pub fn get_nickname(&'this self) -> &'this Nickname<'outer> {
        return &self.nickname;
    }

    pub fn get_passord_hash(&'this self) -> &'this PasswordHash<'outer> {
        return &self.password_hash;
    }

    pub fn get_created_at(&'this self) -> &'this DateTime<'outer> {
        return &self.created_at;
    }

    pub fn get_confirmed(&'this self) -> &'this Confirmed {
        return &self.confirmed;
    }
}