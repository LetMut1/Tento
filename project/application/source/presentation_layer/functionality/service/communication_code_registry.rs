pub struct CommunicationCodeRegistry;

impl CommunicationCodeRegistry {
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__EMAIL_ALREADY_EXIST: &'static str = "apus_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__NICKNAME_ALREADY_EXIST: &'static str = "apus_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__NOT_FOUND: &'static str = "apus_3";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD: &'static str = "apus_4";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_EMAIL: &'static str = "apus_5";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_NICKNAME: &'static str = "apus_6";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_PASSWORD: &'static str = "apus_7";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__NOT_FOUND: &'static str = "apusrecoto_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__INVALID_VALUE: &'static str = "apusrecoto_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__WRONG_VALUE: &'static str = "apusrecoto_3";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__IS_NOT_APPROVED: &'static str = "apusrecoto_4";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__ALREADY_APPROVED: &'static str = "apusrecoto_5";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN__ALREADY_EXPIRED: &'static str = "apusrecoto_6";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND: &'static str = "apusauto_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__INVALID_VALUE: &'static str = "apusauto_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__WRONG_VALUE: &'static str = "apusauto_3";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED: &'static str = "apusauto_4";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND: &'static str = "apusrepato_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__INVALID_VALUE: &'static str = "apusrepato_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE: &'static str = "apusrepato_3";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__IS_NOT_APPROVED: &'static str = "apusrepato_4";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED: &'static str = "apusrepato_5";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED: &'static str = "apusrepato_6";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED: &'static str = "apusacto_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__NOT_EXPIRED: &'static str = "apusacto_2";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST: &'static str = "apusacto_3";

    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__NOT_FOUND: &'static str = "apusacreto_1";
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__ALREADY_EXPIRED: &'static str = "apusacreto_2";
}