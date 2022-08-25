use std::marker::PhantomData;

pub struct ApplicationUserResetPasswordToken {
    application_user_id: i64,
    value: String,
    wrong_enter_tries_quantity: u8,
    is_approved: bool,
    _created_at: PhantomData<String>
}

impl ApplicationUserResetPasswordToken {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u8 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 3;

    pub fn new(
        application_user_id: i64,
        value: String,
        wrong_enter_tries_quantity: u8,
        is_approved: bool,
    ) -> Self {
        return Self {
            application_user_id,
            value,
            wrong_enter_tries_quantity,
            is_approved,
            _created_at: PhantomData
        };
    }

    pub fn get_application_user_id<'a>(
        &'a self
    ) -> i64 {
        return self.application_user_id;
    }

    pub fn get_value<'a>(
        &'a self
    ) -> &'a str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'a>(
        &'a self
    ) -> u8 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_is_approved<'b>(
        &'b self
    ) -> bool {
        return self.is_approved;
    }

    pub fn set_value<'b>(
        &'b mut self,
        value: String
    ) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'a>(
        &'a mut self,
        wrong_enter_tries_quantity: u8
    ) -> &'a mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_is_approved<'b>(
        &'b mut self,
        is_approved: bool
    ) -> &'b mut Self {
        self.is_approved = is_approved;

        return self;
    }
}