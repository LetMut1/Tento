use super::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_event::ApplicationUserLogInTokenWorkflowEvent;
use super::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_workflow_event::ApplicationUserRegistrationConfirmationTokenWorkflowEvent;
use super::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_workflow_event::ApplicationUserResetPasswordTokenWorkflowEvent;
use super::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_workflow_event::ApplicationUserWorkflowEvent;
use super::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_workflow_event::JsonAccessWebTokenWorkflowEvent;
use super::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_workflow_event::JsonRefreshWebTokenWorkflowEvent;

pub enum EntityWorkflowEvent {
    ApplicationUserWorkflowEvent {
        application_user_workflow_event: ApplicationUserWorkflowEvent
    },
    ApplicationUserLogInTokenWorkflowEvent {
        application_user_log_in_token_workflow_event: ApplicationUserLogInTokenWorkflowEvent
    },
    ApplicationUserRegistrationConfirmationTokenWorkflowEvent {
        application_user_registration_confirmation_token_workflow_event: ApplicationUserRegistrationConfirmationTokenWorkflowEvent
    },
    ApplicationUserResetPasswordTokenWorkflowEvent {
        application_user_reset_password_token_workflow_event: ApplicationUserResetPasswordTokenWorkflowEvent
    },
    JsonAccessWebTokenWorkflowEvent {
        json_access_web_token_workflow_event: JsonAccessWebTokenWorkflowEvent
    },
    JsonRefreshWebTokenWorkflowEvent {
        json_refresh_web_token_workflow_event: JsonRefreshWebTokenWorkflowEvent
    }
}