use {
    super::Resolver,
    crate::infrastructure_layer::data::aggregate_error::AggregateError,
    chrono::Utc,
};
pub struct UnixTime;
impl Resolver<UnixTime> {
    pub fn get_now_in_microseconds() -> i64 {
        return Utc::now().timestamp_micros();
    }
    pub fn get_now_in_nanoseconds() -> Result<i64, AggregateError> {
        return crate::option_into_logic_out_of_range!(Utc::now().timestamp_nanos_opt());
    }
    pub fn add_interval_microseconds(quantity_of_microseconds: i64, to: i64) -> Result<i64, AggregateError> {
        return crate::option_into_logic_out_of_range!(to.checked_add(quantity_of_microseconds));
    }
}
