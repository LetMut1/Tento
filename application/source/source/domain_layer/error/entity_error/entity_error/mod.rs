use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_core::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use super::_core::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use super::_core::_in_context_for::domain_layer::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use super::_core::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use super::_core::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use super::_core::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use super::_core::_in_context_for::domain_layer::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error::PreConfirmedApplicationUserError;

#[derive(Debug)]
pub enum EntityError {
    ApplicationUserError(ApplicationUserError),
    ApplicationUserLogInTokenError(ApplicationUserLogInTokenError),
    ApplicationUserRegistrationConfirmationTokenError(ApplicationUserRegistrationConfirmationTokenError),
    JsonAccessWebTokenError(JsonAccessWebTokenError),
    JsonRefreshWebTokenError(JsonRefreshWebTokenError),
    PreConfirmedApplicationUserError(PreConfirmedApplicationUserError),
    ApplicationUserResetPasswordTokenError(ApplicationUserResetPasswordTokenError)
}

impl Display for EntityError {
    fn fmt<'this, 'outer_a>(&'this self, _: &'outer_a mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for EntityError {}