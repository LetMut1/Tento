use super::Logger;
pub use crate::infrastructure_layer::data::{
    error::Error,
};
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
impl Logger<Error> {
    pub fn log<'a>(error: &'a Error) -> () {
        tracing::error!(
            "{}",
            Formatter::<Error>::format(error).as_str(),
        );
        return ();
    }
}