use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::common::Common;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use std::borrow::Cow;
use super::core::value::Value;
use super::core::wrong_enter_tries_quantity::WrongEnterTriesQuanity;

pub struct ApplicationUserResetPasswordToken<'outer_a> {
    application_user_id: &'outer_a ApplicationUserId,
    application_user_email: Cow<'outer_a, Email>,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuanity
}

impl<'this, 'outer_a: 'this> ApplicationUserResetPasswordToken<'outer_a> {
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 5;

    pub fn new(application_user: &'outer_a ApplicationUser<'outer_a>) -> Self {
        return Self {
            application_user_id: application_user.get_id(),
            application_user_email: Cow::Borrowed(application_user.get_email()),
            value: Value::new(UuidV4::new().get_value().to_string()),    // TODO создать генератор значения + метода Рефреш ниже
            wrong_enter_tries_quantity: WrongEnterTriesQuanity::new(0)
        };
    }

    pub fn new_from_model(common: Common<'outer_a>, application_user_id: &'outer_a ApplicationUserId) -> Self {
        return Self {
            application_user_id,
            application_user_email: Cow::Owned(Email::new(common.application_user_email.into_owned())),
            value: Value::new(common.value.into_owned()),
            wrong_enter_tries_quantity: WrongEnterTriesQuanity::new(common.wrong_enter_tries_quantity)
        };
    }

    pub fn get_application_user_id(&'this self) -> &'this ApplicationUserId {
        return self.application_user_id;
    }

    pub fn get_application_user_email(&'this self) -> &'this Email {
        return self.application_user_email.as_ref();
    }

    pub fn get_value(&'this self) -> &'this Value {
        return &self.value;
    }

    pub fn get_wrong_enter_tries_quantity(&'this self) -> &'this WrongEnterTriesQuanity {
        return &self.wrong_enter_tries_quantity;
    }

    pub fn increment_wrong_enter_tries_quantity(&'this mut self) -> &'this mut Self {
        self.wrong_enter_tries_quantity.increment();

        return self;
    }
}