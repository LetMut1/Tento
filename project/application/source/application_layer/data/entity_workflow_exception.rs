#[allow(non_snake_case)]
pub enum EntityWorkflowException {
    ApplicationUser {
        application_user__workflow_exception: ApplicationUser_WorkflowException
    },
    ApplicationUserAuthorizationToken {
        application_user_authorization_token__workflow_exception: ApplicationUserAuthorizationToken_WorkflowException
    },
    ApplicationUserRegistrationToken {
        application_user_registration_token__workflow_exception: ApplicationUserRegistrationToken_WorkflowException
    },
    ApplicationUserResetPasswordToken {
        application_user_reset_password_token__workflow_exception: ApplicationUserResetPasswordToken_WorkflowException
    },
    ApplicationUserAccessToken {
        application_user_access_token__workflow_exception: ApplicationUserAccessToken_WorkflowException
    },
    ApplicationUserAccessRefreshToken {
        application_user_access_refresh_token__workflow_exception: ApplicationUserAccessRefreshToken_WorkflowException
    }
}

#[allow(non_camel_case_types)]
pub enum ApplicationUser_WorkflowException {
    EmailAlreadyExist,
    InvalidEmail,
    InvalidNickname,
    InvalidPassword,
    NicknameAlreadyExist,
    NotFound,
    WrongPassword
}

#[allow(non_camel_case_types)]
pub enum ApplicationUserAccessRefreshToken_WorkflowException {
    NotFound,
    AlreadyExpired
}

#[allow(non_camel_case_types)]
pub enum ApplicationUserAccessToken_WorkflowException {
    AlreadyExpired,
    InApplicationUserAccessTokenBlackList,
    NotExpired
}

#[allow(non_camel_case_types)]
pub enum ApplicationUserAuthorizationToken_WorkflowException {
    AlreadyExpired,
    InvalidValue,
    NotFound,
    WrongValue
}

#[allow(non_camel_case_types)]
pub enum ApplicationUserRegistrationToken_WorkflowException {
    AlreadyApproved,
    AlreadyExpired,
    InvalidValue,
    IsNotApproved,
    NotFound,
    WrongValue
}

#[allow(non_camel_case_types)]
pub enum ApplicationUserResetPasswordToken_WorkflowException {
    AlreadyApproved,
    AlreadyExpired,
    InvalidValue,
    IsNotApproved,
    NotFound,
    WrongValue
}