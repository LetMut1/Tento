use crate::domain_layer::entity::entity::application_user_pre_confirmed::_component::id::Id as ApplicationUserPreConfirmedId;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use std::borrow::Cow;
use super::_component::value::Value;
use super::_component::wrong_enter_tries_quantity::WrongEnterTriesQuanity;


pub struct ApplicationUserRegistrationConfirmationToken<'outer_a> {
    application_user_pre_confirmed_id: &'outer_a ApplicationUserPreConfirmedId,
    application_user_email: Cow<'outer_a, Email>,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuanity
}

impl<'outer_a> ApplicationUserRegistrationConfirmationToken<'outer_a> {
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 5;

    pub fn new(
        application_user_pre_confirmed_id: &'outer_a ApplicationUserPreConfirmedId,
        application_user_email: Cow<'outer_a, Email>,
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuanity
    ) -> Self {
        return Self {
            application_user_pre_confirmed_id, application_user_email, value, wrong_enter_tries_quantity
        };
    }

    pub fn increment_wrong_enter_tries_quantity<'this>(&'this mut self) -> &'this mut Self {
        self.wrong_enter_tries_quantity.increment();

        return self;
    }
    
    pub fn get_application_user_pre_confirmed_id<'this>(&'this self) -> &'this ApplicationUserPreConfirmedId {
        return self.application_user_pre_confirmed_id;
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