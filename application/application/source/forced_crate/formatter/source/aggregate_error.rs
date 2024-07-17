use super::{
    context_report,
    Formatter,
};
use aggregate_error::{
    AggregateError,
    AggregateError_,
    Backtrace,
    InvalidArgument,
    Logic,
    Runtime,
};
impl Formatter<AggregateError> {
    pub fn format<'a>(aggregate_error: &'a AggregateError) -> String {
        let message_part = match aggregate_error.0.subject {
            AggregateError_::Logic {
                ref logic,
            } => Formatter::<Logic>::format(logic),
            AggregateError_::Runtime {
                ref runtime,
            } => Formatter::<Runtime>::format(runtime),
            AggregateError_::InvalidArgument {
                ref invalid_argument,
            } => Formatter::<InvalidArgument>::format(invalid_argument),
        };
        return format!(
            context_report!(),
            message_part.as_str(),
            Formatter::<Backtrace>::format(&aggregate_error.0.backtrace).as_str(),
        );
    }
}
