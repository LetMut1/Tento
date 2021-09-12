use chrono::DateTime;
use chrono::Duration;
use chrono::offset::TimeZone;
use chrono::Utc;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct DateTimeResolver;

impl DateTimeResolver {                     // TODO переделать дженерик на конкретный тип после ухода от прокседТАйпс
    const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.6f%#z";

    pub fn is_greater_or_equal_than<'outer_a, Tz>(subject_date_time: &'outer_a DateTime<Tz>, than_date_time: &'outer_a DateTime<Tz>) -> bool
    where
        Tz: TimeZone            // TODO сделать через UTC !!!!!!!!!!!!!!!!!!!!!!
    {
        return subject_date_time.timestamp() >= than_date_time.timestamp();
    }

    pub fn add_interval_from_now(quantity_of_minutes: i64) -> Result<String, BaseError> {     // TODO Переделать, тут должно быть два параметрв. а Now  подставляем уже в другом сервисе
        match Utc::now().checked_add_signed(Duration::minutes(quantity_of_minutes)) {
            Some(date_time) => {
                return Ok(date_time.format(Self::TIMESTAMP_FORMAT).to_string());
            },
            None => {
                return Err(BaseError::LogicError("Too big date must not be added."));
            }
        };
    }

    pub fn is_valid_timestamp<'outer_a>(timestamp_value: &'outer_a str) -> bool {
        if let Ok(_date_time) = DateTime::parse_from_str(timestamp_value, Self::TIMESTAMP_FORMAT) {
            return true;
        }

        return false;
    }
}