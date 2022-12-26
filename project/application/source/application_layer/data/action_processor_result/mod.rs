use super::entity_workflow_exception::ApplicationUserAccessRefreshToken_WorkflowException;
use super::entity_workflow_exception::ApplicationUserAccessToken_WorkflowException;
use super::entity_workflow_exception::ApplicationUserLogInToken_WorkflowException;
use super::entity_workflow_exception::ApplicationUserRegistrationConfirmationToken_WorkflowException;
use super::entity_workflow_exception::ApplicationUserResetPasswordToken_WorkflowException;
use super::entity_workflow_exception::ApplicationUser_WorkflowException;
use super::entity_workflow_exception::EntityWorkflowException;

pub enum ActionProcessorResult<T> {
    Outcoming {
        outcoming: T
    },
    EntityWorkflowException {
        entity_workflow_exception: EntityWorkflowException
    }
}

impl<T> ActionProcessorResult<T> {
    pub fn new_with_outcoming(
        outcoming: T
    ) -> Self {
        return Self::Outcoming { outcoming };
    }

    #[allow(non_snake_case)]
    pub fn new_with_application_user_workflow_exception(
        application_user__workflow_exception: ApplicationUser_WorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUser { application_user__workflow_exception } };
    }

    #[allow(non_snake_case)]
    pub fn new_with_application_user_log_in_token_workflow_exception(
        application_user_log_in_token__workflow_exception: ApplicationUserLogInToken_WorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserLogInToken { application_user_log_in_token__workflow_exception } };
    }

    #[allow(non_snake_case)]
    pub fn new_with_application_user_registration_confirmation_token_workflow_exception(
        application_user_registration_confirmation_token__workflow_exception: ApplicationUserRegistrationConfirmationToken_WorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserRegistrationConfirmationToken { application_user_registration_confirmation_token__workflow_exception } };
    }

    #[allow(non_snake_case)]
    pub fn new_with_application_user_reset_password_token_workflow_exception(
        application_user_reset_password_token__workflow_exception: ApplicationUserResetPasswordToken_WorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserResetPasswordToken { application_user_reset_password_token__workflow_exception } };
    }

    #[allow(non_snake_case)]
    pub fn new_with_application_user_access_token_workflow_exception(
        application_user_access_token__workflow_exception: ApplicationUserAccessToken_WorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserAccessToken { application_user_access_token__workflow_exception } };
    }

    #[allow(non_snake_case)]
    pub fn new_with_application_user_access_refresh_token_workflow_exception(
        application_user_access_refresh_token__workflow_exception: ApplicationUserAccessRefreshToken_WorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserAccessRefreshToken { application_user_access_refresh_token__workflow_exception } };
    }
}