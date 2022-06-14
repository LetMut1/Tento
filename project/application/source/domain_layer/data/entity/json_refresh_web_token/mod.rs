use std::borrow::Cow;

pub struct JsonRefreshWebToken<'a> {
    json_access_web_token_id: String,
    application_user_id: i64,
    application_user_log_in_token_device_id: Cow<'a, str>,
    obfuscation_value: String
}

impl<'a> JsonRefreshWebToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u16 = 60 * 24 * 30;

    pub fn new(
        json_access_web_token_id: String,
        application_user_id: i64,
        application_user_log_in_token_device_id: Cow<'a, str>,
        obfuscation_value: String
    ) -> Self {
        return Self {
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            obfuscation_value
        };
    }

    pub fn get_json_access_web_token_id<'b>(
        &'b self
    ) -> &'b str {
        return self.json_access_web_token_id.as_str();
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

    pub fn get_obfuscation_value<'b>(
        &'b self
    ) -> &'b str {
        return self.obfuscation_value.as_str();
    }

    pub fn set_obfuscation_value<'b>(
        &'b mut self,
        obfuscation_value: String
    ) -> &'b mut Self {
        self.obfuscation_value = obfuscation_value;

        return self;
    }
}