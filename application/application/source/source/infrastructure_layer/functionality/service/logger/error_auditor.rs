use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::error;
use super::Logger;

pub use crate::infrastructure_layer::data::auditor::Auditor;
pub use crate::infrastructure_layer::data::error::Error;

impl Logger<Auditor<Error>> {
    pub fn log<'a>(
        error_auditor: &'a Auditor<Error>
    ) -> () {
        let message = Formatter::<Auditor<Error>>::format(error_auditor);

        error!("{}", message.as_str());

        return ();
    }
}
