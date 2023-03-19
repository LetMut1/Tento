use std::borrow::Cow;

pub struct ApplicationUserRegistrationToken<'a> {
    application_user_email: Cow<'a, str>,
    application_user_device_id: Cow<'a, str>,
    value: String,
    wrong_enter_tries_quantity: i16,
    is_approved: bool,
    expires_at: i64,
    can_be_resent_from: i64
}

impl<'a> ApplicationUserRegistrationToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i16 = 60 * 3;
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i16 = 1;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 5;

    pub fn new(
        application_user_email: Cow<'a, str>,
        application_user_device_id: Cow<'a, str>,
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64,
        can_be_resent_from: i64
    ) -> Self {
        return Self {
            application_user_email,
            application_user_device_id,
            value,
            wrong_enter_tries_quantity,
            is_approved,
            expires_at,
            can_be_resent_from
        };
    }

    pub fn get_application_user_email<'b>(&'b self) -> &'b str {
        return self.application_user_email.as_ref();
    }

    pub fn get_application_user_device_id<'b>(&'b self) -> &'b str {
        return self.application_user_device_id.as_ref();
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

    pub fn get_can_be_resent_from<'b>(&'b self) -> i64 {
        return self.can_be_resent_from;
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

    pub fn set_can_be_resent_from<'b>(&'b mut self, can_be_resent_from: i64) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}