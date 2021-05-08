use r2d2::Error as R2d2Error;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum ConnectionPoolErrorKind {
    CommonError(R2d2Error)
}

impl Display for ConnectionPoolErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for ConnectionPoolErrorKind {}