use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use tracing::error;
use super::Logger;

pub use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;

impl Logger<ErrorAuditor> {
    pub fn log<'a>(
        error_auditor: &'a ErrorAuditor
    ) -> () {
        let message = Formatter::<ErrorAuditor>::format(error_auditor);

        error!("{}", message.as_str());

        return ();
    }
}
