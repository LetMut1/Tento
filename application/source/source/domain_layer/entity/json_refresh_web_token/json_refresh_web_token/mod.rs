use std::borrow::Cow;
use uuid::Uuid;

pub struct JsonRefreshWebToken<'outer_a> {
    json_access_web_token_id: String,
    application_user_id: Cow<'outer_a, i64>,
    application_user_log_in_token_device_id: Cow<'outer_a, str>,
    obfuscation_value: String
}

impl<'outer_a> JsonRefreshWebToken<'outer_a> {
    pub fn new(
        json_access_web_token_id: String,
        application_user_id: Cow<'outer_a, i64>,
        application_user_log_in_token_device_id: Cow<'outer_a, str>,
        obfuscation_value: String
    ) -> Self {
        return Self {
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            obfuscation_value
        };
    }

    pub fn refresh<'this>(
        &'this mut self
    ) -> &'this mut Self {
        self.obfuscation_value = Uuid::new_v4().to_string();     // TODO Через сервис ПЕРЕДЕЛАТь

        return self;
    }

    pub fn get_json_access_web_token_id<'this>(
        &'this self
    ) -> &'this str {
        return self.json_access_web_token_id.as_str();
    }

    pub fn get_application_user_id<'this>(
        &'this self
    ) -> &'this i64 {
        return self.application_user_id.as_ref();
    }

    pub fn get_application_user_log_in_token_device_id<'this>(
        &'this self
    ) -> &'this str {
        return self.application_user_log_in_token_device_id.as_ref();
    }

    pub fn get_obfuscation_value<'this>(
        &'this self
    ) -> &'this str {
        return self.obfuscation_value.as_str();
    }
}