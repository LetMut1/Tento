use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use crate::infrastructure_layer::error::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;

pub struct DateTimeResolver;

impl DateTimeResolver {
    const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.6f%:z";      // TODO был %#z (Он только для парсинга, судя по документации). Проверить совместимость с Бд.

    pub fn create_chrono_date_time_utc<'a>(
        date_time: &'a str
    ) -> Result<DateTime<Utc>, ErrorAggregator> {
        return Ok(DateTime::parse_from_str(date_time, Self::TIMESTAMP_FORMAT)?.with_timezone(&Utc));
    }

    pub fn is_greater_or_equal_than<'a>(
        subject_date_time: &'a DateTime<Utc>,
        than_date_time: &'a DateTime<Utc>
    ) -> bool {
        return subject_date_time.timestamp() >= than_date_time.timestamp();
    }

    pub fn add_interval_from<'a>(
        date_time: &'a DateTime<Utc>,
        quantity_of_minutes: &'a i64
    ) -> Result<String, ErrorAggregator> {
        match date_time.checked_add_signed(Duration::minutes(*quantity_of_minutes)) {
            Some(date_time) => {
                return Ok(date_time.format(Self::TIMESTAMP_FORMAT).to_string());
            },
            None => {
                return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(false, "Too big date must not be added.")});
            }
        };
    }
    
    pub fn add_interval_from_now<'a>(
        quantity_of_minutes: &'a i64
    ) -> Result<String, ErrorAggregator> {
        return Self::add_interval_from(&Utc::now(), quantity_of_minutes)
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