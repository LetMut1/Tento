use std::borrow::Cow;
use std::clone::Clone;

#[derive(Clone)]
pub struct Payload<'a> {
    id: Cow<'a, str>,
    application_user_id: Cow<'a, i64>,
    application_user_log_in_token_device_id: Cow<'a, str>,
    expiration_time: String
}

impl<'a> Payload<'a> {
    pub fn new(
        id: Cow<'a, str>,
        application_user_id: Cow<'a, i64>,
        application_user_log_in_token_device_id: Cow<'a, str>,
        exp: String
    ) -> Self {
        return Self {
            id,
            application_user_id,
            application_user_log_in_token_device_id,
            expiration_time: exp
        };
    }

    pub fn get_id<'b>(
        &'b self
    ) -> &'b str {
        return self.id.as_ref();
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

    pub fn get_expiration_time<'b>(
        &'b self
    ) -> &'b str {
        return self.expiration_time.as_str();
    }
}