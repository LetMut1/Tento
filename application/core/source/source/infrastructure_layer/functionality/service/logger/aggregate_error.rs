use {
    super::Logger,
    crate::infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::formatter::Formatter,
    },
};
impl Logger<AggregateError> {
    pub fn log<'a>(aggregate_error: &'a AggregateError) -> () {
        tracing::error!(
            "{}",
            Formatter::<AggregateError>::format(aggregate_error).as_str(),
        );
        return ();
    }
}
