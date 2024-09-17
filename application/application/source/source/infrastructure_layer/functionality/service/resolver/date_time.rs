use super::Resolver;
use aggregate_error::{
    AggregateError,
    Backtrace,
    OptionConverter,
};
use chrono::{
    DateTime as ChronoDateTime,
    Utc,
};
pub struct DateTime;
impl Resolver<DateTime> {
    // Rule for 2022-09-18 03:03:39.308889+0000
    const TIMESTAMP_FORMAT_TO_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.6f%z";
    // Rule for 2022-09-18 03:03:39.308889+00
    const TIMESTAMP_FORMAT_TO_PARSE: &'static str = "%Y-%m-%d %H:%M:%S%.6f%#z";
    pub fn unixtime_get_now() -> i64 {
        return Utc::now().timestamp();
    }
    pub fn unixtime_add_minutes_interval_from_now(quantity_of_minutes: i64) -> Result<i64, AggregateError> {
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
    pub fn unixtime_is_greater_or_equal_than_now(unix_time: i64) -> bool {
        return unix_time >= Utc::now().timestamp();
    }
    pub fn timestamp_is_valid_timestamp<'a>(date_time: &'a str) -> bool {
        return ChronoDateTime::parse_from_str(
            date_time,
            Self::TIMESTAMP_FORMAT_TO_PARSE,
        )
        .is_ok();
    }
}
