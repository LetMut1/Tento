pub struct StorageKeyResolver;

impl StorageKeyResolver {
    const PREFIX_REPOSITORY_APPLICATION_USER_LOG_IN_TOKEN_FIRST: &'static str = "r:apuslointo:1:"; 
    const PREFIX_REPOSITORY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST: &'static str = "r:apusrecoto:1:";
    const PREFIX_REPOSITORY_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST: &'static str = "r:apusrepato:1:";
    const PREFIX_REPOSITORY_JSON_ACCESS_WEB_TOKEN_BLACK_LIST_FIRST: &'static str = "r:jsacwetoblli:1:";
    const PREFIX_REPOSITORY_JSON_REFRESH_WEB_TOKEN_FIRST: &'static str = "r:jsreweto:1:";
    const PREFIX_SERVICE_JSON_REFRESH_WEB_TOKEN_FIRST: &'static str = "u:jsreweto:1:";

    pub fn get_repository_application_user_log_in_token_first<'a>(
        application_user_id: &'a i64,
        application_user_log_in_token_device_id: &'a str
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_LOG_IN_TOKEN_FIRST.to_string()
            + application_user_id.to_string().as_str()  + ":" + application_user_log_in_token_device_id;
    }
    
    pub fn get_repository_application_user_registration_confirmation_token_first<'a>(
        application_user_pre_confirmed_id: &'a i64
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST.to_string()
            + application_user_pre_confirmed_id.to_string().as_str();
    }

    pub fn get_repository_application_user_reset_password_token_first<'a>(
        application_user_id: &'a i64,
    ) -> String {
        return Self::PREFIX_REPOSITORY_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST.to_string()
            + application_user_id.to_string().as_str();
    }

    pub fn get_repository_json_access_web_token_bkack_list_first<'a>(
        json_access_web_token_id: &'a str
    ) -> String {
        return Self::PREFIX_REPOSITORY_JSON_ACCESS_WEB_TOKEN_BLACK_LIST_FIRST.to_string() + json_access_web_token_id;
    }

    pub fn get_repository_json_refresh_web_token_first<'a>(
        application_user_id: &'a i64,
        application_user_log_in_token_device_id: &'a str,
    ) -> String {
        return Self::PREFIX_REPOSITORY_JSON_REFRESH_WEB_TOKEN_FIRST.to_string()
            + application_user_id.to_string().as_str() + ":" + application_user_log_in_token_device_id;
    }

    pub fn get_service_json_refresh_web_token_first<'a>(
        application_user_id: &'a i64
    ) -> String {
        return Self::PREFIX_SERVICE_JSON_REFRESH_WEB_TOKEN_FIRST.to_string()
            + application_user_id.to_string().as_str();
    }
}