use crate::data_transfer_object::_in_context_for::_resource::_new_for_context::_in_context_for::entity::entity::application_user::_new_for_context::select::Select;
use crate::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use super::_core::created_at::CreatedAt;
use super::_core::email::Email;
use super::_core::id::Id;
use super::_core::nickname::Nickname;
use super::_core::password_hash::PasswordHash;
use super::_core::password::Password;

pub struct ApplicationUser<'outer_a> {
    id: Id,
    email: Cow<'outer_a, Email>,
    nickname: Nickname,
    password_hash: PasswordHash,
    created_at: CreatedAt
}

impl<'outer_a> ApplicationUser<'outer_a> {
    pub fn new_from_pre_confirmed_application_user(
        pre_confirmed_application_user: &'outer_a PreConfirmedApplicationUser, nickname: Nickname, password: Password
    ) -> Result<Self, BaseError> {
        return Ok(
            Self {
                id: Id::new(),
                email: Cow::Borrowed(pre_confirmed_application_user.get_email()),
                nickname,
                password_hash: PasswordHash::new_from_password(password)?,
                created_at: CreatedAt::new()
            }
        );
    }

    pub fn new_from_resource_model(select: Select) -> Self {
        let (
            id,
            email,
            nickname,
            password_hash,
            created_at
        ) = select.into_inner();

        return Self {
            id: Id::new_from_uuid(id),
            email: Cow::Owned(Email::new(email)),
            nickname: Nickname::new(nickname),
            password_hash: PasswordHash::new(password_hash),
            created_at: CreatedAt::new_from_date_time(created_at)
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

    pub fn set_password<'this>(&'this mut self, password: Password) -> Result<&'this mut Self, BaseError> {
        self.password_hash = PasswordHash::new_from_password(password)?;

        return Ok(self);
    }

    pub fn get_id<'this>(&'this self) -> &'this Id {
        return &self.id;
    }

    pub fn get_email<'this>(&'this self) -> &'this Email {
        return self.email.as_ref();
    }

    pub fn get_nickname<'this>(&'this self) -> &'this Nickname {
        return &self.nickname;
    }

    pub fn get_passord_hash<'this>(&'this self) -> &'this PasswordHash {
        return &self.password_hash;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}