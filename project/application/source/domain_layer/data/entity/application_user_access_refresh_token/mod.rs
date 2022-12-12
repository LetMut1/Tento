use serde::Serialize;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessRefreshToken<'a> {
    application_user_id: i64,
    application_user_log_in_token_device_id: Cow<'a, str>,
    application_user_access_token_id: Cow<'a, str>,
    obfuscation_value: String,
    expires_at: i64,
    updated_at: String
}

impl<'a> ApplicationUserAccessRefreshToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i32 = 60 * 24 * 30 * 3;

    pub fn new(
        application_user_id: i64,
        application_user_log_in_token_device_id: Cow<'a, str>,
        application_user_access_token_id: Cow<'a, str>,
        obfuscation_value: String,
        expires_at: i64,
        updated_at: String
    ) -> Self {
        return Self {
            application_user_id,
            application_user_log_in_token_device_id,
            application_user_access_token_id,
            obfuscation_value,
            expires_at,
            updated_at
        };
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

    pub fn get_application_user_access_token_id<'b>(
        &'b self
    ) -> &'b str {
        return self.application_user_access_token_id.as_ref();
    }

    pub fn get_obfuscation_value<'b>(
        &'b self
    ) -> &'b str {
        return self.obfuscation_value.as_str();
    }

    pub fn get_expires_at<'b>(
        &'b self
    ) -> i64 {
        return self.expires_at;
    }

    pub fn get_updated_at<'b>(
        &'b self
    ) -> &'b str {
        return self.updated_at.as_str();
    }

    pub fn set_application_user_access_token_id<'b >(
        &'b mut self,
        application_user_access_token_id: Cow<'a, str>
    ) -> &'b mut Self {
        self.application_user_access_token_id = application_user_access_token_id;

        return self;
    }

    pub fn set_obfuscation_value<'b>(
        &'b mut self,
        obfuscation_value: String
    ) -> &'b mut Self {
        self.obfuscation_value = obfuscation_value;

        return self;
    }

    pub fn set_expires_at<'b>(
        &'b mut self,
        expires_at: i64
    ) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_updated_at<'b>(
        &'b mut self,
        updated_at: String
    ) -> &'b mut Self {
        self.updated_at = updated_at;

        return self;
    }
}