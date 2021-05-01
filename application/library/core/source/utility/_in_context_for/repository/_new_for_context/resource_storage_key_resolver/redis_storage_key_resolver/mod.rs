use crate::entity::core::uuid_v4::UuidV4;

pub struct RedisStorageKeyResolver;

impl<'outer> RedisStorageKeyResolver {
    const PREFIX_REPOSITORY_APPLICATION_USER_LOG_IN_TOKEN_FIRST: &'static str = "re:apuslointo:first:"; 
    const PREFIX_REPOSITORY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST: &'static str = "re:apusrecoto:first:";
    const PREFIX_REPOSITORY_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST: &'static str = "re:apusrepato:first:";
    const PREFIX_REPOSITORY_JSON_ACCESS_WEB_TOKEN_BLACK_LIST_FIRST: &'static str = "re:jsacwetoblli:first:";
    const PREFIX_REPOSITORY_JSON_REFRESH_WEB_TOKEN_FIRST: &'static str = "re:jsreweto:first:";
    const PREFIX_UTILITY_JSON_REFRESH_WEB_TOKEN_FIRST: &'static str = "ut:jsreweto:first:";
    
    pub fn get_repository_application_user_log_in_token_first(
        application_user_id: &'outer UuidV4, application_user_log_in_token_device_id: &'outer UuidV4
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_LOG_IN_TOKEN_FIRST.to_string()
        + application_user_id.get_value().to_string().as_str() + ":"
        + application_user_log_in_token_device_id.get_value().to_string().as_str();
    }
    
    pub fn get_repository_application_user_registration_confirmation_token_first(
        pre_confirmed_application_user_id: &'outer UuidV4
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST.to_string()
        + pre_confirmed_application_user_id.get_value().to_string().as_str();
    }

    pub fn get_repository_application_user_reset_password_token_first(
        application_user_id: &'outer UuidV4,
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST.to_string()
        + application_user_id.get_value().to_string().as_str();
    }

    pub fn get_repository_json_access_web_token_bkack_list_first(
        json_access_web_token_id: &'outer UuidV4
    ) -> String {
        return Self::PREFIX_REPOSITORY_JSON_ACCESS_WEB_TOKEN_BLACK_LIST_FIRST.to_string()
        + json_access_web_token_id.get_value().to_string().as_str();
    }

    pub fn get_repository_json_refresh_web_token_first(
        application_user_id: &'outer UuidV4, application_user_log_in_token_device_id: &'outer UuidV4,
    ) -> String {
        return Self::PREFIX_REPOSITORY_JSON_REFRESH_WEB_TOKEN_FIRST.to_string()
        + application_user_id.get_value().to_string().as_str() + ":"
        + application_user_log_in_token_device_id.get_value().to_string().as_str();
    }

    pub fn get_utility_json_refresh_web_token_first(application_user_id: &'outer UuidV4) -> String {
        return Self::PREFIX_UTILITY_JSON_REFRESH_WEB_TOKEN_FIRST.to_string()
        + application_user_id.get_value().to_string().as_str();
    }
}