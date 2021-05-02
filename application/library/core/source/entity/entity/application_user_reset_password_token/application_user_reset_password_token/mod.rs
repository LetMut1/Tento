use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::common::Common;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::application_user::core::email::Email;
use crate::error::main_error_kind::core::invalid_argument_error::InvalidArgumentError;
use std::borrow::Cow;
use super::core::value::Value;

pub struct ApplicationUserResetPasswordToken<'outer> {
    application_user_id: Cow<'outer, UuidV4>,
    application_user_email: Cow<'outer, Email>,
    value: Value
}

impl<'this, 'outer: 'this> ApplicationUserResetPasswordToken<'outer> {
    pub fn new(application_user: &'outer ApplicationUser<'outer>) -> Self {
        return Self {
            application_user_id: Cow::Borrowed(application_user.get_id()),
            application_user_email: Cow::Borrowed(application_user.get_email()),
            value: Value::new(UuidV4::new().get_value().to_string())     // TODO создать генератор значения + метода Рефреш ниже
        };
    }

    pub fn new_from_model(common: Common<'outer>) -> Result<Self, InvalidArgumentError> {
        return Ok(
            Self {
                application_user_id: Cow::Owned(UuidV4::new_from_string(common.application_user_id)?),
                application_user_email: Cow::Owned(Email::new(common.application_user_email.into_owned())),
                value: Value::new(common.value.into_owned())
            }
        );
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4 {
        return self.application_user_id.as_ref();
    }

    pub fn get_application_user_email(&'this self) -> &'this Email {
        return self.application_user_email.as_ref();
    }

    pub fn get_value(&'this self) -> &'this Value {
        return &self.value;
    }
}