use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use super::core::_in_context_for::diesel_component::_new_for_context::diesel_error::DieselError;
use super::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use super::core::_in_context_for::utility::email_sender::_new_for_context::email_error_kind::EmailErrorKind;
use super::core::connection_error_kind::connection_error_kind::ConnectionErrorKind;
use super::core::logic_error::LogicError;

#[derive(Debug)]
pub enum MainErrorKind {
    ConnectionError(ConnectionErrorKind),
    DieselErrorKind(DieselError),
    EmailErrorKind(EmailErrorKind),
    EntityErrorKind(EntityErrorKind),
    LogicError(LogicError)
}

impl Display for MainErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for MainErrorKind {}

impl From<DieselError> for MainErrorKind {
    fn from(diesel_error: DieselError) -> Self {
        return Self::DieselErrorKind(diesel_error);
    }
}

impl From<ConnectionErrorKind> for MainErrorKind {
    fn from(connection_error_kind: ConnectionErrorKind) -> Self {
        return Self::ConnectionError(connection_error_kind);
    }
}

impl From<EntityErrorKind> for MainErrorKind {
    fn from(entity_error_kind: EntityErrorKind) -> Self {
        return Self::EntityErrorKind(entity_error_kind);
    }
}

impl From<EmailErrorKind> for MainErrorKind {
    fn from(email_error_kind: EmailErrorKind) -> Self {
        return Self::EmailErrorKind(email_error_kind);
    }
}

impl From<LogicError> for MainErrorKind {
    fn from(logic_error: LogicError) -> Self {
        return Self::LogicError(logic_error);
    }
}