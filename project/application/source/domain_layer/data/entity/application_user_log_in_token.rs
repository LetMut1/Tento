pub struct ApplicationUserLogInToken<'a> {
    application_user_id: i64,
    device_id: &'a str,
    value: String,
    wrong_enter_tries_quantity: u8,
    expires_at: String
}

impl<'a> ApplicationUserLogInToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u8 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: u8 = 5;

    pub fn new(
        application_user_id: i64,
        device_id: &'a str,
        value: String,
        wrong_enter_tries_quantity: u8,
        expires_at: String
    ) -> Self {
        return Self {
            application_user_id,
            device_id,
            value,
            wrong_enter_tries_quantity,
            expires_at
        };
    }

    pub fn get_application_user_id<'b>(
        &'b self
    ) -> i64 {
        return self.application_user_id;
    }

    pub fn get_device_id<'b>(
        &'b self
    ) -> &'a str {
        return self.device_id;
    }

    pub fn get_value<'b>(
        &'b self
    ) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'b>(
        &'b self
    ) -> u8 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_expires_at<'b>(
        &'b self
    ) -> &'b str {
        return self.expires_at.as_str();
    }

    pub fn set_value<'b>(
        &'b mut self,
        value: String
    ) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(
        &'b mut self,
        wrong_enter_tries_quantity: u8
    ) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_expires_at<'b>(
        &'b mut self,
        expires_at: String
    ) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }
}