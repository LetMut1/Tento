use crate::error::context::Context;
use lettre_email::error::Error as LettreEmailError;
use lettre::error::Error as LettreError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum EmailErrorKind {
    Creating(Context<LettreEmailError>),
    Sending(Context<LettreError>)
}

impl Display for EmailErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for EmailErrorKind {}