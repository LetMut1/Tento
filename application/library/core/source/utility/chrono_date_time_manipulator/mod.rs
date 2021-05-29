use chrono::DateTime;
use chrono::offset::TimeZone;

pub struct ChronoDateTimeManipulator;

impl ChronoDateTimeManipulator {
    pub fn is_greater_or_equal_than<'outer_a, Tz>(subject_date_time: &'outer_a DateTime<Tz>, than_date_time: &'outer_a DateTime<Tz>) -> bool
    where
        Tz: TimeZone
    {
        return subject_date_time.timestamp() >= than_date_time.timestamp();
    }
}