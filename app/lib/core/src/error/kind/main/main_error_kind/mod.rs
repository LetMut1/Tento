use crate::error::kind::common::connection::connection_error::ConnectionError;
use crate::error::kind::diesel_component::diesel_error_kind::DieselErrorKind;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum MainErrorKind {
    DieselErrorKind(DieselErrorKind),
    ConnectionError(ConnectionError)
}

impl Display for MainErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for MainErrorKind {}

impl From<DieselErrorKind> for MainErrorKind {
    fn from(previous: DieselErrorKind) -> Self {
        match previous {
            DieselErrorKind::Any(ref _value) => { return Self::DieselErrorKind(previous); }
        };
    }
}

impl From<ConnectionError> for MainErrorKind {
    fn from(previous: ConnectionError) -> Self {
        match previous {
            ConnectionError::Postgresql(ref _value) => { return Self::ConnectionError(previous); }
        };
    }
}