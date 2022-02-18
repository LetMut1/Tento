use crate::domain_layer::entity::json_access_web_token::_component::payload::Payload;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use std::borrow::Cow;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'a>(
        json_access_web_token_id: Cow<'a, str>,
        application_user_id: Cow<'a, i64>,
        application_user_log_in_token_device_id: Cow<'a, str>,
        expiration_time: String
    ) -> JsonAccessWebToken<'a> {
        return JsonAccessWebToken::new(
            Payload::new(
                json_access_web_token_id,
                application_user_id,
                application_user_log_in_token_device_id,
                expiration_time
            )
        );
    }

    fn create_from_json_refresh_web_token<'a>(
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<JsonAccessWebToken<'a>, Self::Error>;
}