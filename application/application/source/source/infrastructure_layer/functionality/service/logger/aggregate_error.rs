use super::Logger;
pub use crate::infrastructure_layer::data::aggregate_error::AggregateError;
use crate::infrastructure_layer::functionality::service::formatter::Formatter_;
impl Logger<AggregateError> {
    pub fn log<'a>(aggregate_error: &'a AggregateError) -> () {
        tracing::error!(
            "{}",
            Formatter_::<AggregateError>::format(aggregate_error).as_str(),
        );
        return ();
    }
}
