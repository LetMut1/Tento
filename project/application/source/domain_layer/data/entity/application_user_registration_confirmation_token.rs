pub struct ApplicationUserRegistrationConfirmationToken<'a> {
    application_user_email: &'a str,
    value: String,
    wrong_enter_tries_quantity: i16,
    is_approved: bool,
    expires_at: i64
}

impl<'a> ApplicationUserRegistrationConfirmationToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i16 = 60 * 3;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 5;

    pub fn new(
        application_user_email: &'a str,
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64
    ) -> Self {
        return Self {
            application_user_email,
            value,
            wrong_enter_tries_quantity,
            is_approved,
            expires_at
        };
    }

    pub fn get_application_user_email<'b>(&'b self) -> &'a str {
        return self.application_user_email;
    }

    pub fn get_value<'b>(&'b self) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'b>(&'b self) -> i16 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_is_approved<'b>(&'b self) -> bool {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> i64 {
        return self.expires_at;
    }

    pub fn set_value<'b>(&'b mut self, value: String) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(&'b mut self, wrong_enter_tries_quantity: i16) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_is_approved<'b>(&'b mut self, is_approved: bool) -> &'b mut Self {
        self.is_approved = is_approved;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: i64) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }
}