use diesel::result::Error as BaseDieselError;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub struct DieselError {
    base_diesel_error: BaseDieselError
}

impl Display for DieselError {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for DieselError {}

impl From<BaseDieselError> for DieselError {
    fn from(diesel_error: BaseDieselError) -> Self {
        return Self {
            base_diesel_error: diesel_error
        };
    }
}