use extern_crate::serde::Serialize;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Deserialize;

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Debug, Serialize)]
#[serde(crate = "extern_crate::serde")]
pub enum Void {}

#[derive(Debug)]
pub struct ErrorVoid(Void);

impl Error for ErrorVoid {}

impl Display for ErrorVoid {
    fn fmt<'a>(&'a self, _: &'a mut Formatter<'_>) -> Result<(), FmtError> {
        return Err(FmtError);
    }
}