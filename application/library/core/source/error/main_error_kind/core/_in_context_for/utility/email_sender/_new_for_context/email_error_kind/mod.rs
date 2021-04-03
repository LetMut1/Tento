use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use std::error::Error;
use std::convert::From;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum EmailErrorKind {
    CanNotCreate(LettreEmailError),
    CanNotSend(LettreSmtpError)
}

impl Display for EmailErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for EmailErrorKind {}

impl From<LettreEmailError> for EmailErrorKind {
    fn from(lettre_email_error: LettreEmailError) -> Self {
        return Self::CanNotCreate(lettre_email_error)
    }
}

impl From<LettreSmtpError> for EmailErrorKind {
    fn from(lettre_smtp_error: LettreSmtpError) -> Self {
        return Self::CanNotSend(lettre_smtp_error)
    }
}