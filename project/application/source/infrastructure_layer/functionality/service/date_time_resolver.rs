use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use extern_crate::chrono::DateTime;
use extern_crate::chrono::Utc;

pub struct DateTimeResolver;

impl DateTimeResolver {
    /// Rule for 2022-09-18 03:03:39.308889+00
    const TIMESTAMP_FORMAT_TO_PARSE: &'static str = "%Y-%m-%d %H:%M:%S%.6f%#z";
    /// Rule for 2022-09-18 03:03:39.308889+0000
    const TIMESTAMP_FORMAT_TO_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.6f%z";

    pub fn add_interval_from_now(quantity_of_minutes: i64) -> Result<i64, ErrorAuditor> {
        let mut quantity_of_seconds = match quantity_of_minutes.checked_mul(60) {
            Some(quantity_of_seconds_) => quantity_of_seconds_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "Too big quantity of minutes must not be added." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        quantity_of_seconds = match Utc::now()
            .timestamp()
            .checked_add(quantity_of_seconds) {
            Some(quantity_of_seconds_) => quantity_of_seconds_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "Too big quantity of minutes must not be added." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(quantity_of_seconds);
    }

    pub fn is_greater_or_equal_than_now<'a>(unix_time: i64) -> bool {
        return unix_time >= Utc::now().timestamp();
    }

    pub fn is_valid_timestamp<'a>(date_time: &'a str) -> bool {
        if let Ok(_) = DateTime::parse_from_str(date_time, Self::TIMESTAMP_FORMAT_TO_PARSE) {
            return true;
        }

        return false;
    }
}