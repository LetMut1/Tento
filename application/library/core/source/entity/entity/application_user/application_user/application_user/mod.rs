use crate::dto::resourse_model::_in_context_for::entity::entity::application_user::application_user::_new_for_context::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use std::borrow::Cow;
use super::core::email::Email;
use super::core::nickname::Nickname;
use super::core::password_hash::PasswordHash;
use super::core::password::Password;

pub struct ApplicationUser<'outer> {
    id: UuidV4,
    email: Cow<'outer, Email>,
    nickname: Nickname,
    password_hash: PasswordHash,
    created_at: DateTime           // TODO  Roles
}

impl<'this, 'outer: 'this> ApplicationUser<'outer> {
    pub fn new_from_pre_confirmed_application_user(
        pre_confirmed_application_user: &'outer PreConfirmedApplicationUser, nickname: Nickname, password: Password
    ) -> Self {
        return Self {
            id: UuidV4::new(),
            email: Cow::Borrowed(pre_confirmed_application_user.get_email()),
            nickname,
            password_hash: PasswordHash::new_from_password(password),
            created_at: DateTime::new()
        };
    }

    pub fn new_from_model(existing: Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.id),
            email: Cow::Owned(Email::new(existing.email)),
            nickname: Nickname::new(existing.nickname),
            password_hash: PasswordHash::new(existing.password_hash),
            created_at: DateTime::new_from_date_time(existing.created_at)
        };
    }

    pub fn set_email(&'this mut self, email: Email) -> &'this mut Self {
        self.email = Cow::Owned(email);

        return self;
    }

    pub fn set_nickname(&'this mut self, nickname: Nickname) -> &'this mut Self {
        self.nickname = nickname;

        return self;
    }

    pub fn set_password(&'this mut self, password: Password) -> &'this mut Self {
        self.password_hash = PasswordHash::new_from_password(password);

        return self;
    }

    pub fn get_id(&'this self) -> &'this UuidV4 {
        return &self.id;
    }

    pub fn get_email(&'this self) -> &'this Email {
        return self.email.as_ref();
    }

    pub fn get_nickname(&'this self) -> &'this Nickname {
        return &self.nickname;
    }

    pub fn get_passord_hash(&'this self) -> &'this PasswordHash {
        return &self.password_hash;
    }

    pub fn get_created_at(&'this self) -> &'this DateTime {
        return &self.created_at;
    }
}