pub enum InvalidArgumentResult<T> {
    Ok {
        subject: T,
    },
    InvalidArgument {
        invalid_argument: InvalidArgument,
    },
}

pub enum InvalidArgument {
    ApplicationUser_AccessModifier,
    ApplicationUser_Email,
    ApplicationUser_Id,
    ApplicationUser_Nickname,
    ApplicationUser_Password,
    ApplicationUser_VisabilityModifier,
    ApplicationUserAccessRefreshTokenEncrypted,
    ApplicationUserAccessTokenEncrypted,
    ApplicationUserAuthorizationToken_Value,
    ApplicationUserDevice_Id,
    ApplicationUserRegistrationToken_Value,
    ApplicationUserResetPasswordToken_Value,
    Channel_Id,
    Channel_Name,
    HttpHeader,
    HttpRoute,
    Limit,
    SearchParameter,
    SortOrderRepresentation,
    Timestamp,
}
