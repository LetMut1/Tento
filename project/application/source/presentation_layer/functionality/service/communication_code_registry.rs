pub struct CommunicationCodeRegistry;

impl CommunicationCodeRegistry {
    /// Codes for Application_user context.
    /// [0, 999]
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__EMAIL_ALREADY_EXIST: i64 = 0;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__NICKNAME_ALREADY_EXIST: i64 = 1;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__NOT_FOUND: i64 = 2;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD: i64 = 3;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_EMAIL: i64 = 4;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_NICKNAME: i64 = 5;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER__INVALID_PASSWORD: i64 = 6;

    /// Codes for Application_user_registration_token context.
    /// [1000, 1999]
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND: i64 = 1000;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_TOKEN__INVALID_VALUE: i64 = 1001;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE: i64 = 1002;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_TOKEN__IS_NOT_APPROVED: i64 = 1003;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_APPROVED: i64 = 1004;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED: i64 = 1005;

    /// Codes for Application_user_authorization_token context.
    /// [2000, 2999]
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND: i64 = 2000;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__INVALID_VALUE: i64 = 2001;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__WRONG_VALUE: i64 = 2002;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED: i64 = 2003;

    /// Codes for Application_user_reset_password_token context.
    /// [3000, 3999]
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND: i64 = 3000;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__INVALID_VALUE: i64 = 3001;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE: i64 = 3002;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__IS_NOT_APPROVED: i64 = 3003;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED: i64 = 3004;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED: i64 = 3005;

    /// Codes for Application_user_access_token context.
    /// [4000, 4999]
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED: i64 = 4000;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__NOT_EXPIRED: i64 = 4001;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST: i64 = 4002;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_TOKEN__WRONG_DESERIALIZED_FORM: i64 = 4003;

    /// Codes for Application_user_access_refresh_token context.
    /// [5000, 5999]
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__NOT_FOUND: i64 = 5000;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__ALREADY_EXPIRED: i64 = 5001;
    #[allow(non_upper_case_globals)]
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__WRONG_DESERIALIZED_FORM: i64 = 5002;
}