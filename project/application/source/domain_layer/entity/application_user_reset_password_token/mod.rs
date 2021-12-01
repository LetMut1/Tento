pub struct ApplicationUserResetPasswordToken<'a> {
    application_user_id: &'a i64,
    value: String,
    wrong_enter_tries_quantity: u8
}

impl<'a> ApplicationUserResetPasswordToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u8 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 5;

    pub fn new(
        application_user_id: &'a i64,
        value: String,
        wrong_enter_tries_quantity: u8
    ) -> Self {
        return Self {
            application_user_id,
            value,
            wrong_enter_tries_quantity
        };
    }

    pub fn get_application_user_id<'b>(
        &'b self
    ) -> &'a i64 {
        return self.application_user_id;
    }

    pub fn get_value<'b>(
        &'b self
    ) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'b>(
        &'b self
    ) -> &'b u8 {
        return &self.wrong_enter_tries_quantity;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(
        &'b mut self,
        wrong_enter_tries_quantity: u8
    ) -> &'b mut Self {

        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }
}