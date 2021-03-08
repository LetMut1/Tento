use crate::error::context::Context;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum EmailErrorKind {
    Creating(Context<LettreEmailError>),
    Sending(Context<LettreError>)
}

impl EmailErrorKind {
    pub fn new_creating(lettre_email_error: LettreEmailError, message: Option<String>) -> Self {
        return Self::Creating(Context::new(Some(lettre_email_error), message));
    }

    pub fn new_sending(lettre_error: LettreError, message: Option<String>) -> Self {
        return Self::Sending(Context::new(Some(lettre_error), message));
    }
}

impl Display for EmailErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for EmailErrorKind {}