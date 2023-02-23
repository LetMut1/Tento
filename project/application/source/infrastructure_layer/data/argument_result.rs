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
    ApplicationUserRegistrationToken_Value,
    ApplicationUserResetPasswordToken_Value,
    HttpHeaders,
    HttpRoute,
    SortOrderRepresentation
}