use super::Logger;
use aggregate_error::AggregateError;
use formatter::Formatter;
impl Logger<AggregateError> {
    pub fn log<'a>(aggregate_error: &'a AggregateError) -> () {
        tracing::error!(
            "{}",
            Formatter::<AggregateError>::format(aggregate_error).as_str(),
        );
        return ();
    }
}
