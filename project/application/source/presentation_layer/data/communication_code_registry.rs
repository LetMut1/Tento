pub struct CommunicationCodeRegistry;

impl CommunicationCodeRegistry {
    /// Codes for Application_user context.
    /// [0, 999]
    pub const APPLICATION_USER__EMAIL_ALREADY_EXIST: i64 = 0;
    pub const APPLICATION_USER__NICKNAME_ALREADY_EXIST: i64 = 1;
    pub const APPLICATION_USER__NOT_FOUND: i64 = 2;
    pub const APPLICATION_USER__WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD: i64 = 3;
    pub const APPLICATION_USER__IS_CHANNEL_OWNER: i64 = 4;

    /// Codes for Application_user_registration_token context.
    /// [1000, 1999]
    pub const APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND: i64 = 1000;
    pub const APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE: i64 = 1001;
    pub const APPLICATION_USER_REGISTRATION_TOKEN__IS_NOT_APPROVED: i64 = 1002;
    pub const APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_APPROVED: i64 = 1003;
    pub const APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED: i64 = 1004;
    pub const APPLICATION_USER_REGISTRATION_TOKEN__TIME_TO_RESEND_HAS_NOT_COME: i64 = 1005;

    /// Codes for Application_user_authorization_token context.
    /// [2000, 2999]
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND: i64 = 2000;
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__WRONG_VALUE: i64 = 2001;
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED: i64 = 2002;
    pub const APPLICATION_USER_AUTHORIZATION_TOKEN__TIME_TO_RESEND_HAS_NOT_COME: i64 = 2003;

    /// Codes for Application_user_reset_password_token context.
    /// [3000, 3999]
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND: i64 = 3000;
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE: i64 = 3001;
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__IS_NOT_APPROVED: i64 = 3002;
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED: i64 = 3003;
    pub const APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED: i64 = 3004;

    /// Codes for Application_user_access_token context.
    /// [4000, 4999]
    pub const APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED: i64 = 4000;
    pub const APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST: i64 = 4001;

    /// Codes for Application_user_access_refresh_token context.
    /// [5000, 5999]
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__NOT_FOUND: i64 = 5000;
    pub const APPLICATION_USER_ACCESS_REFRESH_TOKEN__ALREADY_EXPIRED: i64 = 5001;

    /// Codes for Channel context.
    /// [6000, 6999]
    pub const CHANNEL__NOT_FOUND: i64 = 6000;
    pub const CHANNEL__IS_CLOSED: i64 = 6001;
}