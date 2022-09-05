use super::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_exception::ApplicationUserLogInTokenWorkflowException;
use super::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_workflow_exception::ApplicationUserRegistrationConfirmationTokenWorkflowException;
use super::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_workflow_exception::ApplicationUserResetPasswordTokenWorkflowException;
use super::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use super::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::application_user_access_token_workflow_exception::ApplicationUserAccessTokenWorkflowException;
use super::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::application_user_access_refresh_token_workflow_exception::ApplicationUserAccessRefreshTokenWorkflowException;
use super::entity_workflow_exception::entity_workflow_exception::EntityWorkflowException;

pub enum ActionHandlerResult<T> {
    ActionHandlerOutcomingData {
        action_handler_outcoming_data: T
    },
    EntityWorkflowException {
        entity_workflow_exception: EntityWorkflowException
    }
}

impl<T> ActionHandlerResult<T> {
    pub fn new_with_action_handler_outcoming_data(
        action_handler_outcoming_data: T
    ) -> Self {
        return Self::ActionHandlerOutcomingData { action_handler_outcoming_data };
    }

    pub fn new_with_application_user_workflow_exception(
        application_user_workflow_exception: ApplicationUserWorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserWorkflowException { application_user_workflow_exception } };
    }

    pub fn new_with_application_user_log_in_token_workflow_exception(
        application_user_log_in_token_workflow_exception: ApplicationUserLogInTokenWorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserLogInTokenWorkflowException { application_user_log_in_token_workflow_exception } };
    }

    pub fn new_with_application_user_registration_confirmation_token_workflow_exception(
        application_user_registration_confirmation_token_workflow_exception: ApplicationUserRegistrationConfirmationTokenWorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserRegistrationConfirmationTokenWorkflowException { application_user_registration_confirmation_token_workflow_exception } };
    }

    pub fn new_with_application_user_reset_password_token_workflow_exception(
        application_user_reset_password_token_workflow_exception: ApplicationUserResetPasswordTokenWorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserResetPasswordTokenWorkflowException { application_user_reset_password_token_workflow_exception } };
    }

    pub fn new_with_application_user_access_token_workflow_exception(
        application_user_access_token_workflow_exception: ApplicationUserAccessTokenWorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserAccessTokenWorkflowException { application_user_access_token_workflow_exception } };
    }

    pub fn new_with_application_user_access_refresh_token_workflow_exception(
        application_user_access_refresh_token_workflow_exception: ApplicationUserAccessRefreshTokenWorkflowException
    ) -> Self {
        return Self::EntityWorkflowException { entity_workflow_exception: EntityWorkflowException::ApplicationUserAccessRefreshTokenWorkflowException { application_user_access_refresh_token_workflow_exception } };
    }
}