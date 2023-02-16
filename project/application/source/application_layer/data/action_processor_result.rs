use extern_crate::serde::Serialize;

pub enum ActionProcessorResult<T>
where
    T: Serialize
{
    Outcoming {
        outcoming: T
    },
    UserWorkflowPrecedent {
        user_workflow_precedent: UserWorkflowPrecedent
    }
}

impl<T> ActionProcessorResult<T>
where
    T: Serialize
{
    pub fn outcoming(outcoming: T) -> Self {
        return Self::Outcoming { outcoming };
    }

    pub fn user_workflow_precedent(user_workflow_precedent: UserWorkflowPrecedent) -> Self {
        return Self::UserWorkflowPrecedent { user_workflow_precedent };
    }
}

pub enum UserWorkflowPrecedent {
    ApplicationUser_EmailAlreadyExist,
    ApplicationUser_InvalidEmail,
    ApplicationUser_InvalidNickname,
    ApplicationUser_InvalidPassword,
    ApplicationUser_NicknameAlreadyExist,
    ApplicationUser_NotFound,
    ApplicationUser_WrongPassword,
    ApplicationUserAccessRefreshToken_AlreadyExpired,
    ApplicationUserAccessRefreshToken_NotFound,
    ApplicationUserAccessRefreshToken_WrongDeserializedForm,
    ApplicationUserAccessToken_AlreadyExpired,
    ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
    ApplicationUserAccessToken_NotExpired,
    ApplicationUserAccessToken_WrongDeserializedForm,
    ApplicationUserAuthorizationToken_AlreadyExpired,
    ApplicationUserAuthorizationToken_InvalidValue,
    ApplicationUserAuthorizationToken_NotFound,
    ApplicationUserAuthorizationToken_WrongValue,
    ApplicationUserRegistrationToken_AlreadyApproved,
    ApplicationUserRegistrationToken_AlreadyExpired,
    ApplicationUserRegistrationToken_InvalidValue,
    ApplicationUserRegistrationToken_IsNotApproved,
    ApplicationUserRegistrationToken_NotFound,
    ApplicationUserRegistrationToken_WrongValue,
    ApplicationUserResetPasswordToken_AlreadyApproved,
    ApplicationUserResetPasswordToken_AlreadyExpired,
    ApplicationUserResetPasswordToken_InvalidValue,
    ApplicationUserResetPasswordToken_IsNotApproved,
    ApplicationUserResetPasswordToken_NotFound,
    ApplicationUserResetPasswordToken_WrongValue
}