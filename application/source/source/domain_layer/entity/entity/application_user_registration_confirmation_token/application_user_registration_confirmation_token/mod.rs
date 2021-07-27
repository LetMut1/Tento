use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::_core::id::Id as PreConfirmedApplicationUserId;
use std::borrow::Cow;
use super::_core::value::Value;
use super::_core::wrong_enter_tries_quantity::WrongEnterTriesQuanity;


pub struct ApplicationUserRegistrationConfirmationToken<'outer_a> {
    pre_confirmed_application_user_id: &'outer_a PreConfirmedApplicationUserId,
    application_user_email: Cow<'outer_a, Email>,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuanity
}

impl<'outer_a> ApplicationUserRegistrationConfirmationToken<'outer_a> {
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 5;

    pub fn new(
        pre_confirmed_application_user_id: &'outer_a PreConfirmedApplicationUserId,
        application_user_email: Cow<'outer_a, Email>,
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuanity
    ) -> Self {
        return Self {
            pre_confirmed_application_user_id, application_user_email, value, wrong_enter_tries_quantity
        };
    }

    pub fn increment_wrong_enter_tries_quantity<'this>(&'this mut self) -> &'this mut Self {
        self.wrong_enter_tries_quantity.increment();

        return self;
    }
    
    pub fn get_pre_confirmed_application_user_id<'this>(&'this self) -> &'this PreConfirmedApplicationUserId {
        return self.pre_confirmed_application_user_id;
    }

    pub fn get_application_user_email<'this>(&'this self) -> &'this Email {
        return self.application_user_email.as_ref();
    }

    pub fn get_value<'this>(&'this self) -> &'this Value {
        return &self.value;
    }

    pub fn get_wrong_enter_tries_quantity<'this>(&'this self) -> &'this WrongEnterTriesQuanity {
        return &self.wrong_enter_tries_quantity;
    }
}