
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::obfuscation_value_generator::ObfuscationValueGenerator;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Base;

impl Base {
    pub fn create_from_id_registry<'a>(
        application_user_id: i64,
        application_user_log_in_token_device_id: &'a str
    ) -> ApplicationUserAccessRefreshToken<'a> {
        return ApplicationUserAccessRefreshToken::new(
            Uuid::new_v4().to_string(),
            application_user_id,
            Cow::Borrowed(application_user_log_in_token_device_id),
            ObfuscationValueGenerator::generate()
        );
    }
}