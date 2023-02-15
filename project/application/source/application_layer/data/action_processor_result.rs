use extern_crate::serde::Serialize;
use super::user_workflow_precedent::ApplicationUser_Precedent;
use super::user_workflow_precedent::ApplicationUserAccessRefreshToken_Precedent;
use super::user_workflow_precedent::ApplicationUserAccessToken_Precedent;
use super::user_workflow_precedent::ApplicationUserAuthorizationToken_Precedent;
use super::user_workflow_precedent::ApplicationUserRegistrationToken_Precedent;
use super::user_workflow_precedent::ApplicationUserResetPasswordToken_Precedent;
use super::user_workflow_precedent::UserWorkflowPrecedent;

pub enum ActionProcessorResult<T> {
    Outcoming {
        outcoming: T
    },
    UserWorkflowPrecedent {
        user_workflow_precedent: UserWorkflowPrecedent
    }
}

impl<T> ActionProcessorResult<T> {
    pub fn outcoming(outcoming: T) -> Self {
        return Self::Outcoming { outcoming };
    }

    #[allow(non_snake_case)]
    pub fn application_user__precedent(application_user__precedent: ApplicationUser_Precedent) -> Self {
        return Self::UserWorkflowPrecedent {
            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser { application_user__precedent }
        };
    }

    #[allow(non_snake_case)]
    pub fn application_user_authorization_token__precedent(
        application_user_authorization_token__precedent: ApplicationUserAuthorizationToken_Precedent
    ) -> Self {
        return Self::UserWorkflowPrecedent {
            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAuthorizationToken { application_user_authorization_token__precedent }
        };
    }

    #[allow(non_snake_case)]
    pub fn application_user_registration_token__precedent(
        application_user_registration_token__precedent: ApplicationUserRegistrationToken_Precedent
    ) -> Self {
        return Self::UserWorkflowPrecedent {
            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserRegistrationToken {
                application_user_registration_token__precedent
            }
        };
    }

    #[allow(non_snake_case)]
    pub fn application_user_reset_password_token__precedent(
        application_user_reset_password_token__precedent: ApplicationUserResetPasswordToken_Precedent
    ) -> Self {
        return Self::UserWorkflowPrecedent {
            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserResetPasswordToken {
                application_user_reset_password_token__precedent
            }
        };
    }

    #[allow(non_snake_case)]
    pub fn application_user_access_token__precedent(
        application_user_access_token__precedent: ApplicationUserAccessToken_Precedent
    ) -> Self {
        return Self::UserWorkflowPrecedent {
            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAccessToken { application_user_access_token__precedent }
        };
    }

    #[allow(non_snake_case)]
    pub fn application_user_access_refresh_token__precedent(
        application_user_access_refresh_token__precedent: ApplicationUserAccessRefreshToken_Precedent
    ) -> Self {
        return Self::UserWorkflowPrecedent {
            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAccessRefreshToken {
                application_user_access_refresh_token__precedent
            }
        };
    }
}