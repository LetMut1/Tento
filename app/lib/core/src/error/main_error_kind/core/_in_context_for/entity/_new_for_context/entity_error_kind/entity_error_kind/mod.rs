use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use super::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use super::core::_in_context_for::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use super::core::_in_context_for::entity::pre_registered_application_user_registration_confirmation_token::_new_for_context::pre_registered_application_user_registration_confirmation_token_error_kind::PreRegisteredApplicationUserRegistrationConfirmationTokenErrorKind;

#[derive(Debug)]
pub enum EntityErrorKind {
    ApplicationUserErrorKind(ApplicationUserErrorKind),
    PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind),
    PreRegisteredApplicationUserRegistrationConfirmationTokenErrorKind(PreRegisteredApplicationUserRegistrationConfirmationTokenErrorKind)
}

impl Display for EntityErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for EntityErrorKind {}