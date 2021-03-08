use crate::error::context::Context;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum DieselErrorKind {
    Any(Context<DieselError>)
}

impl DieselErrorKind {
    pub fn new_any(diesel_error: DieselError, message: Option<String>) -> Self {
        return Self::Any(Context::new(Some(diesel_error), message));
    }
}

impl Display for DieselErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for DieselErrorKind {}
