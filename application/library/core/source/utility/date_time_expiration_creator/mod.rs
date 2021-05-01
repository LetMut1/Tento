use crate::entity::core::date_time::DateTime;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;

pub struct DateTimeExpirationCreator;

impl DateTimeExpirationCreator {
    pub const QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST: i64 = 60 * 6;
    pub const QUANTITY_OF_MINUTES_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST: i64 = 60 * 24;
    pub const QUANTITY_OF_MINUTES_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST: i64 = 60;
    pub const QUANTITY_OF_MINUTES_JSON_ACCESS_WEB_TOKEN_FIRST: i64 = 30;
    pub const QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST: i64 = 60 * 24 * 14;
    
    pub fn create_application_user_log_in_token_first() -> DateTime {
        return DateTimeManipulator::add_interval_from_now(Self::QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST);
    }

    pub fn create_application_user_registration_confirmation_token_first() -> DateTime {
        return DateTimeManipulator::add_interval_from_now(Self::QUANTITY_OF_MINUTES_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST);
    }

    pub fn create_application_user_reset_password_token_first() -> DateTime {
        return DateTimeManipulator::add_interval_from_now(Self::QUANTITY_OF_MINUTES_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST);
    }

    pub fn create_json_access_web_token_first() -> DateTime {
        return DateTimeManipulator::add_interval_from_now(Self::QUANTITY_OF_MINUTES_JSON_ACCESS_WEB_TOKEN_FIRST);
    }

    pub fn create_json_refresh_web_token_first() -> DateTime {
        return DateTimeManipulator::add_interval_from_now(Self::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST);
    }
}