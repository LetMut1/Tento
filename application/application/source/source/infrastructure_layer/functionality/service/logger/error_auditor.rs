use super::Logger;
pub use crate::infrastructure_layer::data::{
    auditor::Auditor,
    error::Error,
};
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
impl Logger<Auditor<Error>> {
    pub fn log<'a>(error_auditor: &'a Auditor<Error>) -> () {
        let message = Formatter::<Auditor<Error>>::format(error_auditor);
        tracing::error!("{}", message.as_str());
        return ();
    }
}
