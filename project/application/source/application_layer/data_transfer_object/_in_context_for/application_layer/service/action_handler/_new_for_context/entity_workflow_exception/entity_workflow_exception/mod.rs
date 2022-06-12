use super::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_exception::ApplicationUserLogInTokenWorkflowException;
use super::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_workflow_exception::ApplicationUserRegistrationConfirmationTokenWorkflowException;
use super::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_workflow_exception::ApplicationUserResetPasswordTokenWorkflowException;
use super::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use super::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_workflow_exception::JsonAccessWebTokenWorkflowException;
use super::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_workflow_exception::JsonRefreshWebTokenWorkflowException;

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
    JsonAccessWebTokenWorkflowException {
        json_access_web_token_workflow_exception: JsonAccessWebTokenWorkflowException
    },
    JsonRefreshWebTokenWorkflowException {
        json_refresh_web_token_workflow_exception: JsonRefreshWebTokenWorkflowException
    }
}