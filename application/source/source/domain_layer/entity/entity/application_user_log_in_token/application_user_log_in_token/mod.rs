use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use std::borrow::Cow;
use super::_component::device_id::DeviceId;
use super::_component::value::Value;
use super::_component::wrong_enter_tries_quantity::WrongEnterTriesQuanity;

pub struct ApplicationUserLogInToken<'outer_a> {
    application_user_id: &'outer_a ApplicationUserId,
    device_id: &'outer_a DeviceId,
    application_user_email: Cow<'outer_a, Email>,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuanity
}

impl<'outer_a> ApplicationUserLogInToken<'outer_a> {
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 5;

    pub fn new(
        application_user_id: &'outer_a ApplicationUserId,
        device_id: &'outer_a DeviceId,
        application_user_email: Cow<'outer_a, Email>,
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuanity
    ) -> Self {
        return Self {
            application_user_id, device_id, application_user_email, value, wrong_enter_tries_quantity
        };
    }

    pub fn increment_wrong_enter_tries_quantity<'this>(&'this mut self) -> &'this mut Self {
        self.wrong_enter_tries_quantity.increment();

        return self;
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return self.application_user_id;
    }

    pub fn get_device_id<'this>(&'this self) -> &'this DeviceId {
        return self.device_id;
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