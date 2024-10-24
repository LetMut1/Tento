use super::Resolver;
use crate::infrastructure_layer::data::aggregate_error::{
    AggregateError,
    Backtrace,
    OptionConverter,
};
use chrono::Utc;
pub struct UnixTime;
impl Resolver<UnixTime> {
    pub fn get_now() -> i64 {
        return Utc::now().timestamp();
    }
    pub fn add_minutes_interval_from_now(quantity_of_minutes: i64) -> Result<i64, AggregateError> {
        let quantity_of_seconds = quantity_of_minutes.checked_mul(60).into_logic_out_of_range(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Utc::now().timestamp().checked_add(quantity_of_seconds).into_logic_out_of_range(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
    pub fn is_greater_or_equal_than_now(unix_time: i64) -> bool {
        return unix_time >= Utc::now().timestamp();
    }
}
