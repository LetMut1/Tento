use r2d2::Error as R2d2Error;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ConnectionPoolError {
    CommonError(R2d2Error)
}

impl Display for ConnectionPoolError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ConnectionPoolError {}