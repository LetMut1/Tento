use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_core::other_error::OtherError;
use super::_core::resource_error::resource_error::ResourceError;

#[derive(Debug)]
pub enum RunTimeError {
    OtherError(OtherError),
    ResourceError(ResourceError)
}

impl Display for RunTimeError {
    fn fmt<'this, 'outer_a>(&'this self, _: &'outer_a mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for RunTimeError {}