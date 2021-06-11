use lettre_email::error::Error as EmailError;
use lettre::smtp::error::Error as SmtpError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum EmailServerErrorKind {
    EmailError(EmailError),
    SmtpError(SmtpError)
}

impl Display for EmailServerErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for EmailServerErrorKind {}