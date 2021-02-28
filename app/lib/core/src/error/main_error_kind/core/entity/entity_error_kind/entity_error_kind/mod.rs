use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use super::core::entity::application_user::application_user_error_kind::ApplicationUserErrorKind;

#[derive(Debug)]
pub enum EntityErrorKind {
    ApplicationUserErrorKind(ApplicationUserErrorKind)
}

impl Display for EntityErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for EntityErrorKind {}