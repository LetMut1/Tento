use serde::{
    Deserialize,
    Serialize,
};
use std::{
    error::Error,
    fmt::{
        Display,
        Error as FmtError,
        Formatter,
    },
};
use bitcode::{
    Encode,
    Decode,
};
// Empty type.
#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum Void {}
impl Error for Void {}
impl Display for Void {
    fn fmt<'a>(&'a self, _: &'a mut Formatter<'_>) -> Result<(), FmtError> {
        return Result::Ok(());
    }
}
