use extern_crate::lettre_email::error::Error as EmailError;
use extern_crate::lettre::smtp::error::Error as SmtpError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum EmailServerError {
    EmailError {
        email_error: EmailError
    },
    SmtpError {
        smtp_error: SmtpError
    }
}

impl Display for EmailServerError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}