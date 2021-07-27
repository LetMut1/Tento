use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use super::_core::created_at::CreatedAt;
use super::_core::email::Email;
use super::_core::id::Id;
use super::_core::nickname::Nickname;
use super::_core::password_hash::PasswordHash;

pub struct ApplicationUser<'outer_a> {
    id: Option<Id>,
    email: Cow<'outer_a, Email>,
    nickname: Nickname,
    password_hash: PasswordHash,
    created_at: CreatedAt
}

impl<'outer_a> ApplicationUser<'outer_a> {
    pub fn new(
        id: Option<Id>, email: Cow<'outer_a, Email>, nickname: Nickname, password_hash: PasswordHash, created_at: CreatedAt
    ) -> Self {
        return Self {
            id, email, nickname, password_hash, created_at 
        };
    }

    pub fn set_email<'this>(&'this mut self, email: Email) -> &'this mut Self {
        self.email = Cow::Owned(email);

        return self;
    }

    pub fn set_nickname<'this>(&'this mut self, nickname: Nickname) -> &'this mut Self {
        self.nickname = nickname;

        return self;
    }

    pub fn set_password_hash<'this>(&'this mut self, password_hash: PasswordHash) -> &'this mut Self {
        self.password_hash = password_hash;

        return self;
    }

    pub fn get_id<'this>(&'this self) -> Result<&'this Id, BaseError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(BaseError::LogicError("Id does not exist yet."))
            }
        }
    }

    pub fn get_email<'this>(&'this self) -> &'this Email {
        return self.email.as_ref();
    }

    pub fn get_nickname<'this>(&'this self) -> &'this Nickname {
        return &self.nickname;
    }

    pub fn get_password_hash<'this>(&'this self) -> &'this PasswordHash {
        return &self.password_hash;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}