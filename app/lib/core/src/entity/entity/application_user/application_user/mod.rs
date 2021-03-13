use crate::dto::resourse_model::_in_context_for::entity::entity::application_user::_new_for_context::existing::Existing;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::core::confirmed::Confirmed;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::entity::entity::application_user::core::password_hash::PasswordHash;
use crate::entity::entity::application_user::core::password::Password;

pub struct ApplicationUser {
    id: UuidV4,
    email: Email,
    nickname: Nickname,
    password_hash: PasswordHash,
    created_at: DateTime,           // TODO  Roles
    confirmed: Confirmed
}

impl<'this> ApplicationUser {
    pub fn new(email: Email, nickname: Nickname, password: Password) -> Self {
        return Self {
            id: UuidV4::new(),
            email,
            nickname,
            password_hash: PasswordHash::new_from_password(password),
            created_at: DateTime::new(),
            confirmed: Confirmed::new(false)
        };
    }

    pub fn new_from_model(existing: Existing) -> Self {
        return Self {
            id: UuidV4::new_from_uuid(existing.id),
            email: Email::new(existing.email),
            nickname: Nickname::new(existing.nickname),
            password_hash: PasswordHash::new(existing.password_hash),
            created_at: DateTime::new_from_date_time(existing.created_at),
            confirmed: Confirmed::new(existing.confirmed)
        };
    }

    pub fn is_confirmed(&'this self) -> bool {
        return (&self.confirmed).get_value();
    }

    pub fn set_email(&'this mut self, email: Email) -> &'this mut Self {
        self.email = email;

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
        return &self.email;
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

    pub fn get_confirmed(&'this self) -> &'this Confirmed {
        return &self.confirmed;
    }
}