use super::core::common::connection::connection_error::ConnectionError;
use super::core::diesel_component::diesel_error_kind::DieselErrorKind;
use super::core::entity::entity_error_kind::EntityErrorKind;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum MainErrorKind {
    DieselErrorKind(DieselErrorKind),
    ConnectionError(ConnectionError),
    EntityErrorKind(EntityErrorKind)

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

impl From<EntityErrorKind> for MainErrorKind {
    fn from(previous: EntityErrorKind) -> Self {
        match previous {
            EntityErrorKind::ApplicationUserErrorKind(ref _value) => { return Self::EntityErrorKind(previous); }
        };
    }
}