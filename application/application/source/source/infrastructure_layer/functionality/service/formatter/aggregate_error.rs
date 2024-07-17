use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::aggregate_error::AggregateError;
impl Formatter<AggregateError> {
    pub fn format<'a>(aggregate_error: &'a AggregateError) -> String {
        return Formatter_::<AggregateError>::format(aggregate_error);
    }
}
