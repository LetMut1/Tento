use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;

pub struct DateTimeResolver;

impl DateTimeResolver {
    const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.6f%:z";      // TODO был %#z (Он только для парсинга, судя по документации). Проверить совместимость с Бд.

    pub fn create_chrono_date_time_utc<'a>(
        date_time: &'a str
    ) -> Result<DateTime<Utc>, ErrorAuditor> {
        match DateTime::parse_from_str(date_time, Self::TIMESTAMP_FORMAT) {
            Ok(date_time_) => {
                return Ok(date_time_.with_timezone(&Utc));
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn is_greater_or_equal_than<'a>(
        subject_date_time: &'a DateTime<Utc>,
        than_date_time: &'a DateTime<Utc>
    ) -> bool {
        return subject_date_time.timestamp() >= than_date_time.timestamp();
    }

    pub fn add_interval_from<'a>(
        date_time: &'a DateTime<Utc>,
        quantity_of_minutes: i64
    ) -> Result<DateTime<Utc>, ErrorAuditor> {
        match date_time.checked_add_signed(Duration::minutes(quantity_of_minutes)) {
            Some(date_time_) => {
                return Ok(date_time_);
            }
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "Too big date must not be added.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
    }

    pub fn add_interval_from_now_formated(
        quantity_of_minutes: i64
    ) -> Result<String, ErrorAuditor> {
        match Self::add_interval_from(&Utc::now(), quantity_of_minutes) {
            Ok(date_time) => {
                return Ok(date_time.format(Self::TIMESTAMP_FORMAT).to_string())
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }

    pub fn is_valid_timestamp<'a>(
        date_time: &'a str
    ) -> bool {
        if let Ok(_date_time) = DateTime::parse_from_str(date_time, Self::TIMESTAMP_FORMAT) {
            return true;
        }

        return false;
    }
}