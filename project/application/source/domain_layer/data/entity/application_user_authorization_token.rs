pub struct ApplicationUserAuthorizationToken<'a> {
    application_user_id: i64,
    application_user_device_id: &'a str,
    value: String,
    wrong_enter_tries_quantity: i16,
    expires_at: i64
}

impl<'a> ApplicationUserAuthorizationToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i16 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 5;

    pub fn new(
        application_user_id: i64,
        application_user_device_id: &'a str,
        value: String,
        wrong_enter_tries_quantity: i16,
        expires_at: i64
    ) -> Self {
        return Self {
            application_user_id,
            application_user_device_id,
            value,
            wrong_enter_tries_quantity,
            expires_at
        };
    }

    pub fn get_application_user_id<'b>(&'b self) -> i64 {
        return self.application_user_id;
    }

    pub fn get_application_user_device_id<'b>(&'b self) -> &'a str {
        return self.application_user_device_id;
    }

    pub fn get_value<'b>(&'b self) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'b>(&'b self) -> i16 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_expires_at<'b>(&'b self) -> i64 {
        return self.expires_at;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(&'b mut self, wrong_enter_tries_quantity: i16) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: i64) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }
}