pub struct CommunicationCodeRegistry;

impl CommunicationCodeRegistry {
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__EMAIL_ALREADY_EXIST: &'static str = "enapus_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__NICKNAME_ALREADY_EXIST: &'static str = "enapus_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__NOT_FOUND: &'static str = "enapus_3";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD: &'static str = "enapus_4";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_EMAIL: &'static str = "enapus_5";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_NICKNAME: &'static str = "enapus_6";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_PASSWORD: &'static str = "enapus_7";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__NOT_FOUND: &'static str = "enapusrecoto_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__INVALID_VALUE: &'static str = "enapusrecoto_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__WRONG_VALUE: &'static str = "enapusrecoto_3";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__IS_NOT_APPROVED: &'static str = "enapusrecoto_4";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__ALREADY_APPROVED: &'static str = "enapusrecoto_5";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__ALREADY_EXPIRED: &'static str = "enapusrecoto_6";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_LOG_IN_TOKEN__NOT_FOUND: &'static str = "enapuslointo_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_LOG_IN_TOKEN__INVALID_VALUE: &'static str = "enapuslointo_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_LOG_IN_TOKEN__WRONG_VALUE: &'static str = "enapuslointo_3";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_LOG_IN_TOKEN__ALREADY_EXPIRED: &'static str = "enapuslointo_4";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND: &'static str = "enapusrepato_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__INVALID_VALUE: &'static str = "enapusrepato_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE: &'static str = "enapusrepato_3";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__IS_NOT_APPROVED: &'static str = "enapusrepato_4";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED: &'static str = "enapusrepato_5";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED: &'static str = "enapusrepato_6";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED: &'static str = "enapusacto_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__NOT_EXPIRED: &'static str = "enapusacto_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST: &'static str = "enapusacto_3";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__NOT_FOUND: &'static str = "enapusacreto_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__ALREADY_EXPIRED: &'static str = "enapusacreto_2";
}