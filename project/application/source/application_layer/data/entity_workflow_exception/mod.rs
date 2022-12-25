pub enum EntityWorkflowException {
    ApplicationUserWorkflowException {
        application_user_workflow_exception: ApplicationUserWorkflowException
    },
    ApplicationUserLogInTokenWorkflowException {
        application_user_log_in_token_workflow_exception: ApplicationUserLogInTokenWorkflowException
    },
    ApplicationUserRegistrationConfirmationTokenWorkflowException {
        application_user_registration_confirmation_token_workflow_exception: ApplicationUserRegistrationConfirmationTokenWorkflowException
    },
    ApplicationUserResetPasswordTokenWorkflowException {
        application_user_reset_password_token_workflow_exception: ApplicationUserResetPasswordTokenWorkflowException
    },
    ApplicationUserAccessTokenWorkflowException {
        application_user_access_token_workflow_exception: ApplicationUserAccessTokenWorkflowException
    },
    ApplicationUserAccessRefreshTokenWorkflowException {
        application_user_access_refresh_token_workflow_exception: ApplicationUserAccessRefreshTokenWorkflowException
    }
}

pub enum ApplicationUserWorkflowException {
    EmailAlreadyExist,
    InvalidEmail,
    InvalidNickname,
    InvalidPassword,
    NicknameAlreadyExist,
    NotFound,
    WrongPassword
}

pub enum ApplicationUserAccessRefreshTokenWorkflowException {
    NotFound,
    AlreadyExpired
}

pub enum ApplicationUserAccessTokenWorkflowException {
    AlreadyExpired,
    InApplicationUserAccessTokenBlackList,
    NotExpired
}

pub enum ApplicationUserLogInTokenWorkflowException {
    AlreadyExpired,
    InvalidValue,
    NotFound,
    WrongValue
}

pub enum ApplicationUserRegistrationConfirmationTokenWorkflowException {
    AlreadyApproved,
    AlreadyExpired,
    InvalidValue,
    IsNotApproved,
    NotFound,
    WrongValue
}

pub enum ApplicationUserResetPasswordTokenWorkflowException {
    AlreadyApproved,
    AlreadyExpired,
    InvalidValue,
    IsNotApproved,
    NotFound,
    WrongValue
}