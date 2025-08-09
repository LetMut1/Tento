use std::{
    error::Error,
    fmt::{
        Display,
        Error as FmtError,
        Formatter,
    },
};
// Empty type.
// It is necessary for architecture, because it is impossible to use "! (NeverType)" in Option<!>, for example..
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum Void {}
impl Error for Void {}
impl Display for Void {
    fn fmt<'a>(&'a self, _: &'a mut Formatter<'_>) -> Result<(), FmtError> {
        return Result::Ok(());
    }
}
