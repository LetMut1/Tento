use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use super::core::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token::ApplicationUserLogInTokenErrorKind;
use super::core::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error_kind::ApplicationUserRegistrationConfirmationTokenErrorKind;
use super::core::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error_kind::ApplicationUserResetPasswordTokenErrorKind;
use super::core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use super::core::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use super::core::_in_context_for::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error_kind::JsonAccessWebTokenErrorKind;
use super::core::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error_kind::JsonRefreshWebTokenErrorKind;

#[derive(Debug)]
pub enum EntityErrorKind {
    ApplicationUserErrorKind(ApplicationUserErrorKind),
    ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind),
    ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind),
    JsonAccessWebTokenErrorKind(JsonAccessWebTokenErrorKind),
    JsonRefreshWebTokenErrorKind(JsonRefreshWebTokenErrorKind),
    PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind),
    ApplicationUserResetPasswordTokenErrorKind(ApplicationUserResetPasswordTokenErrorKind)
}

impl Display for EntityErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for EntityErrorKind {}