use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use extern_crate::chrono::DateTime;
use extern_crate::chrono::Duration;
use extern_crate::chrono::Utc;

pub struct DateTimeResolver;

impl DateTimeResolver {     // TODO TODO  TODO  TODO  TODO  НУЖНО, ЧТОБЫ В БД был вот такой формат 2022-09-18 03:03:39.308889+0000.
    /// Rule for 2022-09-18 03:03:39.308889+00
    const TIMESTAMP_FORMAT_TO_PARSE: &'static str = "%Y-%m-%d %H:%M:%S%.6f%#z";
    /// Rule for 2022-09-18 03:03:39.308889+0000
    const TIMESTAMP_FORMAT_TO_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.6f%z";

    pub fn create_chrono_date_time_utc<'a>(date_time: &'a str) -> Result<DateTime<Utc>, ErrorAuditor> {
        let date_time_ = match DateTime::parse_from_str(date_time, Self::TIMESTAMP_FORMAT_TO_PARSE) {
            Ok(date_time__) => date_time__,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(date_time_.with_timezone(&Utc));
    }

    pub fn is_greater_or_equal_than<'a>(subject_date_time: &'a DateTime<Utc>, than_date_time: &'a DateTime<Utc>) -> bool {
        return subject_date_time.timestamp() >= than_date_time.timestamp();
    }

    pub fn add_interval_from<'a>(date_time: &'a DateTime<Utc>, quantity_of_minutes: i64) -> Result<DateTime<Utc>, ErrorAuditor> {
        let date_time_ = match date_time.checked_add_signed(Duration::minutes(quantity_of_minutes)) {
            Some(date_time__) => date_time__,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "Too big date must not be added.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(date_time_);
    }

    pub fn add_interval_from_now_formated(quantity_of_minutes: i64) -> Result<String, ErrorAuditor> {
        let date_time = match Self::add_interval_from(&Utc::now(), quantity_of_minutes) {
            Ok(date_time_) => date_time_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(date_time.format(Self::TIMESTAMP_FORMAT_TO_FORMAT).to_string())
    }

    pub fn is_valid_timestamp<'a>(date_time: &'a str) -> bool {
        if let Ok(_) = DateTime::parse_from_str(date_time, Self::TIMESTAMP_FORMAT_TO_PARSE) {
            return true;
        }

        return false;
    }

    pub fn add_interval_from_now(quantity_of_minutes: i64) -> Result<i64, ErrorAuditor> {
        let mut quantity_of_seconds = match quantity_of_minutes.checked_mul(60) {
            Some(quantity_of_seconds_) => quantity_of_seconds_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "Too big quantity of minutes must not be added.") },
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
                        BaseError::LogicError { logic_error: LogicError::new(false, "Too big quantity of minutes must not be added.") },
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
}