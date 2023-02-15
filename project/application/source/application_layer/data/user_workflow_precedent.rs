pub enum UserWorkflowPrecedent {
    ApplicationUser {
        application_user__precedent: ApplicationUser_Precedent
    },
    ApplicationUserAuthorizationToken {
        application_user_authorization_token__precedent: ApplicationUserAuthorizationToken_Precedent
    },
    ApplicationUserRegistrationToken {
        application_user_registration_token__precedent: ApplicationUserRegistrationToken_Precedent
    },
    ApplicationUserResetPasswordToken {
        application_user_reset_password_token__precedent: ApplicationUserResetPasswordToken_Precedent
    },
    ApplicationUserAccessToken {
        application_user_access_token__precedent: ApplicationUserAccessToken_Precedent
    },
    ApplicationUserAccessRefreshToken {
        application_user_access_refresh_token__precedent: ApplicationUserAccessRefreshToken_Precedent
    }
}

pub enum ApplicationUser_Precedent {
    EmailAlreadyExist,
    InvalidEmail,
    InvalidNickname,
    InvalidPassword,
    NicknameAlreadyExist,
    NotFound,
    WrongPassword
}

pub enum ApplicationUserAccessRefreshToken_Precedent {
    AlreadyExpired,
    NotFound,
    WrongDeserializedForm
}

pub enum ApplicationUserAccessToken_Precedent {
    AlreadyExpired,
    InApplicationUserAccessTokenBlackList,
    NotExpired,
    WrongDeserializedForm
}

pub enum ApplicationUserAuthorizationToken_Precedent {
    AlreadyExpired,
    InvalidValue,
    NotFound,
    WrongValue
}

pub enum ApplicationUserRegistrationToken_Precedent {
    AlreadyApproved,
    AlreadyExpired,
    InvalidValue,
    IsNotApproved,
    NotFound,
    WrongValue
}

pub enum ApplicationUserResetPasswordToken_Precedent {
    AlreadyApproved,
    AlreadyExpired,
    InvalidValue,
    IsNotApproved,
    NotFound,
    WrongValue
}