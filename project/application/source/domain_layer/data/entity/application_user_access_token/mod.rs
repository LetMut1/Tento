use std::borrow::Cow;

pub struct ApplicationUserAccessToken<'a> {
    id: Cow<'a, str>,
    application_user_id: i64,
    application_user_log_in_token_device_id: Cow<'a, str>,
    // obfuscation_value: String,       // TODO Все равно добавить сюда. Проверить еще раз
    expiration_time: String
}

impl<'a> ApplicationUserAccessToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u8 = 30;

    pub fn new(
        id: Cow<'a, str>,
        application_user_id: i64,
        application_user_log_in_token_device_id: Cow<'a, str>,
        expiration_time: String
    ) -> Self {
        return Self {
            id,
            application_user_id,
            application_user_log_in_token_device_id,
            expiration_time
        };
    }

    pub fn get_id<'b>(
        &'b self
    ) -> &'b str {
        return self.id.as_ref();
    }

    pub fn get_application_user_id<'b>(
        &'b self
    ) -> i64 {
        return self.application_user_id;
    }

    pub fn get_application_user_log_in_token_device_id<'b>(
        &'b self
    ) -> &'b str {
        return self.application_user_log_in_token_device_id.as_ref();
    }

    pub fn get_expiration_time<'b>(
        &'b self
    ) -> &'b str {
        return self.expiration_time.as_str();
    }
}