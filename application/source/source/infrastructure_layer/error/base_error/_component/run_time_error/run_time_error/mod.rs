use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::other_error::OtherError;
use super::_component::resource_error::resource_error::ResourceError;

#[derive(Debug)]
pub enum RunTimeError {
    OtherError {
        other_error: OtherError 
    },
    ResourceError {
        resource_error: ResourceError
    }
}

impl Display for RunTimeError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for RunTimeError {}