use serde::Serialize;
use super::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_event::ApplicationUserLogInTokenWorkflowEvent;
use super::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_workflow_event::ApplicationUserRegistrationConfirmationTokenWorkflowEvent;
use super::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_workflow_event::ApplicationUserResetPasswordTokenWorkflowEvent;
use super::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_workflow_event::ApplicationUserWorkflowEvent;
use super::entity_workflow_event::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_workflow_event::JsonAccessWebTokenWorkflowEvent;
use super::entity_workflow_event::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_workflow_event::JsonRefreshWebTokenWorkflowEvent;
use super::entity_workflow_event::entity_workflow_event::EntityWorkflowEvent;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

pub enum ActionHandlerResult<T> {
    ActionHandlerOutcomingData {
        action_handler_outcoming_data: T
    },
    EntityWorkflowEvent {
        entity_workflow_event: EntityWorkflowEvent
    }
}

impl<T> ActionHandlerResult<T> {
    pub fn new_with_application_user_workflow_event(
        application_user_workflow_event: ApplicationUserWorkflowEvent
    ) -> Self {
        return Self::EntityWorkflowEvent { entity_workflow_event: EntityWorkflowEvent::ApplicationUserWorkflowEvent { application_user_workflow_event } };
    }

    pub fn new_with_application_user_log_in_token_workflow_event(
        application_user_log_in_token_workflow_event: ApplicationUserLogInTokenWorkflowEvent
    ) -> Self {
        return Self::EntityWorkflowEvent { entity_workflow_event: EntityWorkflowEvent::ApplicationUserLogInTokenWorkflowEvent { application_user_log_in_token_workflow_event } };
    }

    pub fn new_with_application_user_registration_confirmation_token_workflow_event(
        application_user_registration_confirmation_token_workflow_event: ApplicationUserRegistrationConfirmationTokenWorkflowEvent
    ) -> Self {
        return Self::EntityWorkflowEvent { entity_workflow_event: EntityWorkflowEvent::ApplicationUserRegistrationConfirmationTokenWorkflowEvent { application_user_registration_confirmation_token_workflow_event } };
    }

    pub fn new_with_application_user_reset_password_token_workflow_event(
        application_user_reset_password_token_workflow_event: ApplicationUserResetPasswordTokenWorkflowEvent
    ) -> Self {
        return Self::EntityWorkflowEvent { entity_workflow_event: EntityWorkflowEvent::ApplicationUserResetPasswordTokenWorkflowEvent { application_user_reset_password_token_workflow_event } };
    }

    pub fn new_with_json_access_web_token_workflow_event(
        json_access_web_token_workflow_event: JsonAccessWebTokenWorkflowEvent
    ) -> Self {
        return Self::EntityWorkflowEvent { entity_workflow_event: EntityWorkflowEvent::JsonAccessWebTokenWorkflowEvent { json_access_web_token_workflow_event } };
    }

    pub fn new_with_json_refresh_web_token_workflow_event(
        json_refresh_web_token_workflow_event: JsonRefreshWebTokenWorkflowEvent
    ) -> Self {
        return Self::EntityWorkflowEvent { entity_workflow_event: EntityWorkflowEvent::JsonRefreshWebTokenWorkflowEvent { json_refresh_web_token_workflow_event } };
    }
}

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
impl<S> ActionHandlerResult<S>
where
    S: Serialize
{
    pub fn new_with_action_handler_outcoming_data(
        action_handler_outcoming_data: S
    ) -> Self {
        return Self::ActionHandlerOutcomingData { action_handler_outcoming_data };
    }
}

#[cfg(feature="facilitate_non_automatic_functional_testing")]
impl<S> ActionHandlerResult<S>
where
    S: Serialize + for<'de> Deserialize<'de>
{
    pub fn new_with_action_handler_outcoming_data(
        action_handler_outcoming_data: S
    ) -> Self {
        return Self::ActionHandlerOutcomingData { action_handler_outcoming_data };
    }
}