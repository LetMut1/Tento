pub struct CommunicationCodeStorage;

impl CommunicationCodeStorage {
    pub const ENTITY_APPLICATION_USER_ALREADY_EXIST: &'static str = "enapus01";
    pub const ENTITY_APPLICATION_USER_NOT_FOUND: &'static str = "enapus02";
    pub const ENTITY_APPLICATION_USER_WRONG_PASSWORD: &'static str = "enapus03";
    pub const ENTITY_APPLICATION_USER_INVALID_EMAIL: &'static str = "enapus04";

    pub const ENTITY_PRE_CONFIRMED_APPLICATION_USER_ALREADY_EXIST: &'static str = "enprcoapus01";
    pub const ENTITY_PRE_CONFIRMED_APPLICATION_USER_NOT_FOUND: &'static str = "enprcoapus02";
    pub const ENTITY_PRE_CONFIRMED_APPLICATION_USER_ALREADY_CONFIRMED: &'static str = "enprcoapus03";

    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND: &'static str = "enapusrecoto02";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE: &'static str = "enapusrecoto03";

    pub const ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND: &'static str = "enapuslointo02";
    pub const ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE: &'static str = "enapuslointo03";

    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND: &'static str = "enapusrepato02";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE: &'static str = "enapusrepato03";

    pub const ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED: &'static str = "enjsacweto03";
    pub const ENTITY_JSON_ACCESS_WEB_TOKEN_NOT_EXPIRED: &'static str = "enjsacweto04";

    pub const ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND: &'static str = "enjsreweto02";

    pub const _COMMON_EMAIL_SENDING_PROBLEM: &'static str = "emse01";
}