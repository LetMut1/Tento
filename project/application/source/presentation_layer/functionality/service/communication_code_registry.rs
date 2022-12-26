pub struct CommunicationCodeRegistry;

impl CommunicationCodeRegistry {
    pub const ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST: &'static str = "enapus_1";
    pub const ENTITY_APPLICATION_USER_NICKNAME_ALREADY_EXIST: &'static str = "enapus_2";
    pub const ENTITY_APPLICATION_USER_NOT_FOUND: &'static str = "enapus_3";
    pub const ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD: &'static str = "enapus_4";
    pub const ENTITY_APPLICATION_USER_INVALID_EMAIL: &'static str = "enapus_5";
    pub const ENTITY_APPLICATION_USER_INVALID_NICKNAME: &'static str = "enapus_6";
    pub const ENTITY_APPLICATION_USER_INVALID_PASSWORD: &'static str = "enapus_7";

    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND: &'static str = "enapusrecoto_1";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE: &'static str = "enapusrecoto_2";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_WRONG_VALUE: &'static str = "enapusrecoto_3";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_IS_NOT_APPROVED: &'static str = "enapusrecoto_4";
    pub const ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_ALREADY_APPROVED: &'static str = "enapusrecoto_5";

    pub const ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND: &'static str = "enapuslointo_1";
    pub const ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE: &'static str = "enapuslointo_2";
    pub const ENTITY_APPLICATION_USER_LOG_IN_TOKEN_WRONG_VALUE: &'static str = "enapuslointo_3";

    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND: &'static str = "enapusrepato_1";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE: &'static str = "enapusrepato_2";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_WRONG_VALUE: &'static str = "enapusrepato_3";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_IS_NOT_APPROVED: &'static str = "enapusrepato_4";
    pub const ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_ALREADY_APPROVED: &'static str = "enapusrepato_5";

    pub const ENTITY_APPLICATION_USER_ACCESS_TOKEN_ALREADY_EXPIRED: &'static str = "enapusacto_1";
    pub const ENTITY_APPLICATION_USER_ACCESS_TOKEN_NOT_EXPIRED: &'static str = "enapusacto_2";
    pub const ENTITY_APPLICATION_USER_ACCESS_TOKEN_IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST: &'static str = "enapusacto_3";

    pub const ENTITY_APPLICATION_USER_ACCESS_REFRESH_TOKEN_NOT_FOUND: &'static str = "enapusacreto_1";
    pub const ENTITY_APPLICATION_USER_ACCESS_REFRESH_TOKEN_ALREADY_EXPIRED: &'static str = "enapusacreto_2";
}