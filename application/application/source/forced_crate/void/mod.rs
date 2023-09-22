use serde::Serialize;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter as StdFormatter;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum Void {}

#[derive(Debug)]
pub struct ErrorVoid(Void);

impl Error for ErrorVoid {}

impl Display for ErrorVoid {
    fn fmt<'a>(
        &'a self,
        _: &'a mut StdFormatter<'_>,
    ) -> Result<(), FmtError> {
        return Err(FmtError);
    }
}