use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessToken<'a> {
    id: Cow<'a, str>,
    application_user_id: i64,
    application_user_log_in_token_device_id: Cow<'a, str>,
    // obfuscation_value: String,       // TODO Все равно добавить сюда. Проверить еще раз
    expires_at: String    // TODO TODO TODO ВОт здесь, скорее всего, нужно передвать ЮниксТайм, так как так легче будет переводить в тайм.
}

impl<'a> ApplicationUserAccessToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: u8 = 30;

    pub fn new(
        id: Cow<'a, str>,
        application_user_id: i64,
        application_user_log_in_token_device_id: Cow<'a, str>,
        expires_at: String
    ) -> Self {
        return Self {
            id,
            application_user_id,
            application_user_log_in_token_device_id,
            expires_at
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

    pub fn get_expires_at<'b>(
        &'b self
    ) -> &'b str {
        return self.expires_at.as_str();
    }
}