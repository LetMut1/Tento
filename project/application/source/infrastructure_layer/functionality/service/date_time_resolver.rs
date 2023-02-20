use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use extern_crate::chrono::Utc;

pub struct DateTimeResolver;

impl DateTimeResolver {
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
}