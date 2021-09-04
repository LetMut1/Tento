use chrono::DateTime;
use chrono::offset::TimeZone;

pub struct DateTimeResolver;

impl DateTimeResolver { // TODO переделать дженерик на конкретный тип после ухода от прокседТАйпс
    pub fn is_greater_or_equal_than<'outer_a, Tz>(subject_date_time: &'outer_a DateTime<Tz>, than_date_time: &'outer_a DateTime<Tz>) -> bool
    where
        Tz: TimeZone
    {
        return subject_date_time.timestamp() >= than_date_time.timestamp();
    }

    pub fn is_valid_timestamp<'outer_a>(timestamp_value: &'outer_a str) -> bool {
        if let Ok(_date_time) = DateTime::parse_from_rfc3339(timestamp_value) {
            return true;
        }

        return false;
    }
}