use std::borrow::Cow;
use std::clone::Clone;

#[derive(Clone)]
pub struct Payload<'outer_a> {
    id: Cow<'outer_a, str>,
    application_user_id: Cow<'outer_a, i64>,
    application_user_log_in_token_device_id: Cow<'outer_a, str>,
    expiration_time: String
}

impl<'outer_a> Payload<'outer_a> {
    pub fn new(
        id: Cow<'outer_a, str>,
        application_user_id: Cow<'outer_a, i64>,
        application_user_log_in_token_device_id: Cow<'outer_a, str>,
        exp: String
    ) -> Self {
        return Self {
            id,
            application_user_id,
            application_user_log_in_token_device_id,
            expiration_time: exp
        };
    }

    pub fn get_id<'this>(
        &'this self
    ) -> &'this str {
        return self.id.as_ref();
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

    pub fn get_expiration_time<'this>(
        &'this self
    ) -> &'this str {
        return self.expiration_time.as_str();
    }
}