use crate::domain_layer::error::logic_error::LogicError;
use std::borrow::Cow;

pub struct ApplicationUserResetPasswordToken<'outer_a> {
    application_user_id: &'outer_a i64,
    application_user_email: Cow<'outer_a, str>,
    value: String,
    wrong_enter_tries_quantity: u8
}

impl<'outer_a> ApplicationUserResetPasswordToken<'outer_a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u8 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 5;

    pub fn new(
        application_user_id: &'outer_a i64,
        application_user_email: Cow<'outer_a, str>,
        value: String,
        wrong_enter_tries_quantity: u8
    ) -> Self {
        return Self {
            application_user_id,
            application_user_email,
            value,
            wrong_enter_tries_quantity
        };
    }

    pub fn increment_wrong_enter_tries_quantity<'this>(
        &'this mut self
    ) -> Result<&'this mut Self, LogicError> {
        if self.wrong_enter_tries_quantity == u8::max_value() {
            return Err(LogicError::new("Out of range for `u8` type."));
        }

        self.wrong_enter_tries_quantity = self.wrong_enter_tries_quantity + 1;

        return Ok(self);
    }

    pub fn get_application_user_id<'this>(
        &'this self
    ) -> &'this i64 {
        return self.application_user_id;
    }

    pub fn get_application_user_email<'this>(
        &'this self
    ) -> &'this str {
        return self.application_user_email.as_ref();
    }

    pub fn get_value<'this>(
        &'this self
    ) -> &'this str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'this>(
        &'this self
    ) -> &'this u8 {
        return &self.wrong_enter_tries_quantity;
    }
}