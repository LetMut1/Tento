pub struct CommunicationCodeRegistry;

impl CommunicationCodeRegistry {
    pub const ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST: &'static str = "enapus01";
    pub const ENTITY_APPLICATION_USER_NICKNAME_ALREADY_EXIST: &'static str = "enapus02";
    pub const ENTITY_APPLICATION_USER_NOT_FOUND: &'static str = "enapus03";
    pub const ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD: &'static str = "enapus04";
    pub const ENTITY_APPLICATION_USER_INVALID_EMAIL: &'static str = "enapus05";
    pub const ENTITY_APPLICATION_USER_INVALID_NICKNAME: &'static str = "enapus06";
    pub const ENTITY_APPLICATION_USER_INVALID_PASSWORD: &'static str = "enapus07";

    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND: &'static str = "enapusrecoto02";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE: &'static str = "enapusrecoto03";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_WRONG_VALUE: &'static str = "enapusrecoto04";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_IS_NOT_APPROVED: &'static str = "enapusrecoto05";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_ALREADY_APPROVED: &'static str = "enapusrecoto06";

    pub const ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND: &'static str = "enapuslointo02";
    pub const ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE: &'static str = "enapuslointo03";
    pub const ENTITY_APPLICATION_USER_LOG_IN_TOKEN_WRONG_VALUE: &'static str = "enapuslointo04";

    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND: &'static str = "enapusrepato02";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE: &'static str = "enapusrepato03";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_WRONG_VALUE: &'static str = "enapusrepato04";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_IS_NOT_APPROVED: &'static str = "enapusrepato05";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_ALREADY_APPROVED: &'static str = "enapusrepato06";

    pub const ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED: &'static str = "enjsacweto03";
    pub const ENTITY_JSON_ACCESS_WEB_TOKEN_NOT_EXPIRED: &'static str = "enjsacweto04";
    pub const ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST: &'static str = "enjsacweto05";

    pub const ENTITY_JSON_REFRESH_WEB_TOKEN_NOT_FOUND: &'static str = "enjsreweto02";
}