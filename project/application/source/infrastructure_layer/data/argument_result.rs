pub enum ArgumentResult<T> {
    Ok {
        subject: T
    },
    InvalidArgument {
        invalid_argument: InvalidArgument
    }
}

pub enum InvalidArgument {
    ApplicationUser_Email,
    ApplicationUser_Nickname,
    ApplicationUser_Password,
    ApplicationUserAccessRefreshToken_DeserializedForm,
    ApplicationUserAccessToken_DeserializedForm,
    ApplicationUserAuthorizationToken_Value,
    ApplicationUserDevice_Id,
    ApplicationUserRegistrationToken_Value,
    ApplicationUserResetPasswordToken_Value,
    Channel_Name,
    HttpHeaders,
    HttpRoute,
    Limit,
    SortOrderRepresentation,
    Timestamp
}