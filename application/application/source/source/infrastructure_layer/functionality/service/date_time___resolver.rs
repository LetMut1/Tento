use super::resolver::Resolver;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use extern_crate::chrono::DateTime as ChronoDateTime;
use extern_crate::chrono::Utc;

pub use crate::infrastructure_layer::data::control_type::DateTime;

impl Resolver<DateTime> {
    /// Rule for 2022-09-18 03:03:39.308889+00
    const TIMESTAMP_FORMAT_TO_PARSE: &'static str = "%Y-%m-%d %H:%M:%S%.6f%#z";
    /// Rule for 2022-09-18 03:03:39.308889+0000
    const TIMESTAMP_FORMAT_TO_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.6f%z";

    pub fn unixtime_get_now() -> i64 {
        return Utc::now().timestamp();
    }

    pub fn unixtime_add_minutes_interval_from_now(quantity_of_minutes: i64) -> Result<i64, ErrorAuditor> {
        let mut quantity_of_seconds = match quantity_of_minutes.checked_mul(60) {
            Some(quantity_of_seconds_) => quantity_of_seconds_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError {
                            message: "Too big quantity of minutes must not be added.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        quantity_of_seconds = match Utc::now().timestamp().checked_add(quantity_of_seconds) {
            Some(quantity_of_seconds_) => quantity_of_seconds_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError {
                            message: "Too big quantity of minutes must not be added.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        return Ok(quantity_of_seconds);
    }

    pub fn unixtime_is_greater_or_equal_than_now(unix_time: i64) -> bool {
        return unix_time >= Utc::now().timestamp();
    }

    pub fn timestamp_is_valid_timestamp<'a>(date_time: &'a str) -> bool {
        if let Ok(_) = ChronoDateTime::parse_from_str(
            date_time,
            Self::TIMESTAMP_FORMAT_TO_PARSE,
        ) {
            return true;
        }

        return false;
    }
}
