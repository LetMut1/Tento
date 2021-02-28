use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum EntityErrorKind {

}   // TODO нужны ли во внутренних !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

impl Display for EntityErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for EntityErrorKind {}