use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use extern_crate::serde::Serialize;

pub enum ActionProcessorResult<T>
where
    T: Serialize
{
    Empty,
    Outcoming {
        outcoming: T
    },
    UserWorkflowPrecedent {
        user_workflow_precedent: UserWorkflowPrecedent
    },
    InvalidArgument {
        invalid_argument: InvalidArgument
    }
}

pub enum UserWorkflowPrecedent {
    ApplicationUser_EmailAlreadyExist,
    ApplicationUser_NicknameAlreadyExist,
    ApplicationUser_NotFound,
    ApplicationUser_WrongPassword,
    ApplicationUserAccessRefreshToken_AlreadyExpired,
    ApplicationUserAccessRefreshToken_NotFound,
    ApplicationUserAccessToken_AlreadyExpired,
    ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
    ApplicationUserAccessToken_NotExpired,
    ApplicationUserAuthorizationToken_AlreadyExpired,
    ApplicationUserAuthorizationToken_NotFound,
    ApplicationUserAuthorizationToken_WrongValue,
    ApplicationUserRegistrationToken_AlreadyApproved,
    ApplicationUserRegistrationToken_AlreadyExpired,
    ApplicationUserRegistrationToken_IsNotApproved,
    ApplicationUserRegistrationToken_NotFound,
    ApplicationUserRegistrationToken_WrongValue,
    ApplicationUserResetPasswordToken_AlreadyApproved,
    ApplicationUserResetPasswordToken_AlreadyExpired,
    ApplicationUserResetPasswordToken_IsNotApproved,
    ApplicationUserResetPasswordToken_NotFound,
    ApplicationUserResetPasswordToken_WrongValue
}