use crate::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use crate::entity::entity::json_access_web_token::_core::payload::_core::id::Id as JsonAccessWebTokenId;
use crate::entity::entity::pre_confirmed_application_user::_core::id::Id as PreConfirmedApplicationUserId;

pub struct RedisStorageKeyResolver;

impl RedisStorageKeyResolver {
    const PREFIX_REPOSITORY_APPLICATION_USER_LOG_IN_TOKEN_FIRST: &'static str = "r:apuslointo:1:"; 
    const PREFIX_REPOSITORY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST: &'static str = "r:apusrecoto:1:";
    const PREFIX_REPOSITORY_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST: &'static str = "r:apusrepato:1:";
    const PREFIX_REPOSITORY_JSON_ACCESS_WEB_TOKEN_BLACK_LIST_FIRST: &'static str = "r:jsacwetoblli:1:";
    const PREFIX_REPOSITORY_JSON_REFRESH_WEB_TOKEN_FIRST: &'static str = "r:jsreweto:1:";
    const PREFIX_UTILITY_JSON_REFRESH_WEB_TOKEN_FIRST: &'static str = "u:jsreweto:1:";
    
    pub fn get_repository_application_user_log_in_token_first<'outer_a>(
        application_user_id: &'outer_a ApplicationUserId, application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_LOG_IN_TOKEN_FIRST.to_string()
        + application_user_id.to_string().as_str() + ":"
        + application_user_log_in_token_device_id.to_string().as_str();
    }
    
    pub fn get_repository_application_user_registration_confirmation_token_first<'outer_a>(
        pre_confirmed_application_user_id: &'outer_a PreConfirmedApplicationUserId
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST.to_string()
        + pre_confirmed_application_user_id.to_string().as_str();
    }

    pub fn get_repository_application_user_reset_password_token_first<'outer_a>(
        application_user_id: &'outer_a ApplicationUserId,
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST.to_string()
        + application_user_id.to_string().as_str();
    }

    pub fn get_repository_json_access_web_token_bkack_list_first<'outer_a>(
        json_access_web_token_id: &'outer_a JsonAccessWebTokenId
    ) -> String {
        return Self::PREFIX_REPOSITORY_JSON_ACCESS_WEB_TOKEN_BLACK_LIST_FIRST.to_string()
        + json_access_web_token_id.to_string().as_str();
    }

    pub fn get_repository_json_refresh_web_token_first<'outer_a>(
        application_user_id: &'outer_a ApplicationUserId, application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId,
    ) -> String {
        return Self::PREFIX_REPOSITORY_JSON_REFRESH_WEB_TOKEN_FIRST.to_string()
        + application_user_id.to_string().as_str() + ":"
        + application_user_log_in_token_device_id.to_string().as_str();
    }

    pub fn get_utility_json_refresh_web_token_first<'outer_a>(application_user_id: &'outer_a ApplicationUserId) -> String {
        return Self::PREFIX_UTILITY_JSON_REFRESH_WEB_TOKEN_FIRST.to_string()
        + application_user_id.to_string().as_str();
    }
}