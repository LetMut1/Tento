use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use super::core::invalid_argument_error::InvalidArgumentError;
use super::core::logic_error::LogicError;
use super::core::resource_error_kind::resource_error_kind::ResourceErrorKind;

#[derive(Debug)]
pub enum MainErrorKind {
    EntityErrorKind(EntityErrorKind),
    InvalidArgumentError,
    LogicError(LogicError),
    ResourceErrorKind(ResourceErrorKind)
}

impl Display for MainErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for MainErrorKind {}

impl From<EntityErrorKind> for MainErrorKind {
    fn from(entity_error_kind: EntityErrorKind) -> Self {
        return Self::EntityErrorKind(entity_error_kind);
    }
}

impl From<InvalidArgumentError> for MainErrorKind {
    fn from(_: InvalidArgumentError) -> Self {
        return Self::InvalidArgumentError;
    }
}

impl From<LogicError> for MainErrorKind {
    fn from(logic_error: LogicError) -> Self {
        return Self::LogicError(logic_error);
    }
}

impl From<ResourceErrorKind> for MainErrorKind {
    fn from(resource_error_kind: ResourceErrorKind) -> Self {
        return Self::ResourceErrorKind(resource_error_kind);
    }
}