use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use super::_component::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::application_user_pre_confirmed_error::ApplicationUserPreConfirmedError;
use super::_component::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use super::_component::_in_context_for::domain_layer::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use super::_component::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use super::_component::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use super::_component::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;

#[derive(Debug)]
pub enum EntityError {
    ApplicationUserError(ApplicationUserError),
    ApplicationUserLogInTokenError(ApplicationUserLogInTokenError),
    ApplicationUserPreConfirmedError(ApplicationUserPreConfirmedError),
    ApplicationUserRegistrationConfirmationTokenError(ApplicationUserRegistrationConfirmationTokenError),
    ApplicationUserResetPasswordTokenError(ApplicationUserResetPasswordTokenError),
    JsonAccessWebTokenError(JsonAccessWebTokenError),
    JsonRefreshWebTokenError(JsonRefreshWebTokenError)
}

impl Display for EntityError {
    fn fmt<'this, 'outer_a>(&'this self, _: &'outer_a mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for EntityError {}