use super::core::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use super::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use super::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum MainErrorKind {
    DieselErrorKind(DieselErrorKind),
    ConnectionError(ConnectionErrorKind),
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

impl From<ConnectionErrorKind> for MainErrorKind {
    fn from(previous: ConnectionErrorKind) -> Self {
        match previous {
            ConnectionErrorKind::Postgresql(ref _value) => { return Self::ConnectionError(previous); }
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