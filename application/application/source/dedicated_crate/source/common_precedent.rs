pub enum CommonPrecedent {
    User_EmailAlreadyExist,
    User_IsChannelOwner,
    User_NicknameAlreadyExist,
    User_NotFound,
    User_WrongEmailOrNicknameOrPassword,
    User_WrongPassword,
    UserAccessRefreshToken_AlreadyExpired,
    UserAccessRefreshToken_NotFound,
    UserAccessToken_AlreadyExpired,
    UserAccessToken_InUserAccessTokenBlackList,
    UserAuthorizationToken_AlreadyExpired,
    UserAuthorizationToken_NotFound,
    UserAuthorizationToken_TimeToResendHasNotCome,
    UserAuthorizationToken_WrongValue,
    UserRegistrationToken_AlreadyApproved,
    UserRegistrationToken_AlreadyExpired,
    UserRegistrationToken_IsNotApproved,
    UserRegistrationToken_NotFound,
    UserRegistrationToken_TimeToResendHasNotCome,
    UserRegistrationToken_WrongValue,
    UserResetPasswordToken_AlreadyApproved,
    UserResetPasswordToken_AlreadyExpired,
    UserResetPasswordToken_IsNotApproved,
    UserResetPasswordToken_NotFound,
    UserResetPasswordToken_TimeToResendHasNotCome,
    UserResetPasswordToken_WrongValue,
    Channel_IsClose,
    Channel_LinkedNameAlreadyExist,
    Channel_NameAlreadyExist,
    Channel_NotFound,
}