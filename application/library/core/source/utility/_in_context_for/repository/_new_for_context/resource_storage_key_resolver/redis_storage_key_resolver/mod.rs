use crate::entity::core::uuid_v4::UuidV4;

pub struct RedisStorageKeyResolver;

impl<'outer> RedisStorageKeyResolver {
    const APPLICATION_USER_LOG_IN_TOKEN_BASE_REPOSITORY_FIRST_KEY_PREFIX: &'static str = "en:apuslointo:";

    const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_BASE_REPOSITORY_FIRST_KEY_PREFIX: &'static str = "en:apusrecoto:";

    const APPLICATION_USER_RESET_PASSWORD_TOKEN_BASE_REPOSITORY_FIRST_KEY_PREFIX: &'static str = "en:apusrepato:";
    
    pub fn get_first_for_application_user_log_in_token_base_repository(
        application_user_id: &'outer UuidV4, application_user_log_in_token_device_id: &'outer UuidV4
    ) -> String {
        return Self::APPLICATION_USER_LOG_IN_TOKEN_BASE_REPOSITORY_FIRST_KEY_PREFIX.to_string()
        + application_user_id.get_value().to_string().as_str() + ":"
        + application_user_log_in_token_device_id.get_value().to_string().as_str();
    }
    
    pub fn get_first_for_application_user_registration_confirmation_token_base_repository(
        pre_confirmed_application_user_id: &'outer UuidV4
    ) -> String {
        return Self::APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_BASE_REPOSITORY_FIRST_KEY_PREFIX.to_string()
        + pre_confirmed_application_user_id.get_value().to_string().as_str();
    }

    pub fn get_first_for_application_user_reset_password_token_base_repository(
        application_user_id: &'outer UuidV4,
    ) -> String {
        return Self::APPLICATION_USER_RESET_PASSWORD_TOKEN_BASE_REPOSITORY_FIRST_KEY_PREFIX.to_string()
        + application_user_id.get_value().to_string().as_str();
    }
}