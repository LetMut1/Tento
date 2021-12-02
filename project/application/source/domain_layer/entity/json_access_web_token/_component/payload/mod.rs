use std::borrow::Cow;
use std::clone::Clone;

#[derive(Clone)]
pub struct Payload<'a> {
    json_access_web_token_id: Cow<'a, str>,
    application_user_id: Cow<'a, i64>,
    application_user_log_in_token_device_id: Cow<'a, str>,
    json_access_web_token_expiration_time: String
}

impl<'a> Payload<'a> {
    pub fn new(
        json_access_web_token_id: Cow<'a, str>,
        application_user_id: Cow<'a, i64>,
        application_user_log_in_token_device_id: Cow<'a, str>,
        json_access_web_token_expiration_time: String
    ) -> Self {
        return Self {
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            json_access_web_token_expiration_time
        };
    }

    pub fn get_json_access_web_token_id<'b>(
        &'b self
    ) -> &'b str {
        return self.json_access_web_token_id.as_ref();
    }

    pub fn get_application_user_id<'b>(
        &'b self
    ) -> &'b i64 {
        return self.application_user_id.as_ref();
    }

    pub fn get_application_user_log_in_token_device_id<'b>(
        &'b self
    ) -> &'b str {
        return self.application_user_log_in_token_device_id.as_ref();
    }

    pub fn get_json_access_web_token_expiration_time<'b>(
        &'b self
    ) -> &'b str {
        return self.json_access_web_token_expiration_time.as_str();
    }
}