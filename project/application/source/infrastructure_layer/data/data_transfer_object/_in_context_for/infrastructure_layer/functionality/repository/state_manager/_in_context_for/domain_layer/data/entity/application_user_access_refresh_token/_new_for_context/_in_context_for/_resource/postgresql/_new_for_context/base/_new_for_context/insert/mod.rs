use std::borrow::Cow;

pub struct Insert<'a> {
    application_user_access_token_id: String,
    application_user_id: i64,
    application_user_log_in_token_device_id: Cow<'a, str>,
    obfuscation_value: String,
}

impl<'a> Insert<'a> {
    pub fn new(
        application_user_access_token_id: String,
        application_user_id: i64,
        application_user_log_in_token_device_id: Cow<'a, str>,
        obfuscation_value: String,
    ) -> Self {
        return Self {
            application_user_access_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            obfuscation_value
        }
    }

    pub fn into_inner(
        self
    ) -> (String, i64, Cow<'a, str>, String) {
        return (
            self.application_user_access_token_id,
            self.application_user_id,
            self.application_user_log_in_token_device_id,
            self.obfuscation_value
        );
    }
}