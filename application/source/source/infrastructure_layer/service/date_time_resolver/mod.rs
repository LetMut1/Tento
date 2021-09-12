use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct DateTimeResolver;

impl DateTimeResolver {
    const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.6f%#z";

    pub fn create_chrono_date_time_utc<'outer_a>(
        date_time: &'outer_a str
    ) -> Result<DateTime<Utc>, BaseError> {
        return Ok(DateTime::parse_from_str(date_time, Self::TIMESTAMP_FORMAT)?.with_timezone(&Utc));
    }

    pub fn is_greater_or_equal_than<'outer_a>(
        subject_date_time: &'outer_a DateTime<Utc>,
        than_date_time: &'outer_a DateTime<Utc>
    ) -> bool {
        return subject_date_time.timestamp() >= than_date_time.timestamp();
    }

    pub fn add_interval_from<'outer_a>(
        date_time: &'outer_a DateTime<Utc>,
        quantity_of_minutes: &'outer_a i64
    ) -> Result<String, BaseError> {
        match date_time.checked_add_signed(Duration::minutes(*quantity_of_minutes)) {
            Some(date_time) => {
                return Ok(date_time.format(Self::TIMESTAMP_FORMAT).to_string());
            },
            None => {
                return Err(BaseError::LogicError("Too big date must not be added."));
            }
        };
    }
    
    pub fn add_interval_from_now<'outer_a>(
        quantity_of_minutes: &'outer_a i64
    ) -> Result<String, BaseError> {
        return Self::add_interval_from(&Utc::now(), quantity_of_minutes)
    }

    pub fn is_valid_timestamp<'outer_a>(
        date_time: &'outer_a str
    ) -> bool {
        if let Ok(_date_time) = DateTime::parse_from_str(date_time, Self::TIMESTAMP_FORMAT) {
            return true;
        }

        return false;
    }
}