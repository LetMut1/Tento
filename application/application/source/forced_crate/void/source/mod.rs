use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
#[derive(Debug, Serialize, Deserialize)]
pub enum Void {}
impl Error for Void {}
impl Display for Void {
    fn fmt<'a>(
        &'a self,
        _: &'a mut Formatter<'_>,
    ) -> Result<(), FmtError> {
        return Ok(());
    }
}
