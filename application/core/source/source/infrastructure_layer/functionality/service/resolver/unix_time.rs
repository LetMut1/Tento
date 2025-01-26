use super::Resolver;
use crate::infrastructure_layer::data::aggregate_error::AggregateError;
use chrono::Utc;
pub struct UnixTime;
impl Resolver<UnixTime> {
    pub fn get_now() -> i64 {
        return Utc::now().timestamp();
    }
    pub fn add_interval(quantity_of_seconds: i64, to: i64) -> Result<i64, AggregateError> {
        return crate::option_into_result_logic_out_of_range!(to.checked_add(quantity_of_seconds));
    }
}
