use chrono::Duration;
use chrono::offset::Utc;
use crate::entity::_core::date_time::DateTime;
use crate::utility::chrono_date_time_manipulator::ChronoDateTimeManipulator;

pub struct DateTimeManipulator;

impl DateTimeManipulator {
    // pub fn add_interval<'outer_a>(date_time: &'outer_a DateTime, quantity_of_minutes: i64) -> () {              // TODO написать тесты. проверить, создатся ли интервал. Сделегировать на utility_chrono_date_ime_manipulator?
    //     date_time.get_value().checked_add_signed(Duration::minutes(quantity_of_minutes)).unwrap();  // TODO unwrap

    //     return ();
    // }

    pub fn add_interval_from_now(quantity_of_minutes: i64) -> DateTime {
        return DateTime::new_from_date_time(Utc::now().checked_add_signed(Duration::minutes(quantity_of_minutes)).unwrap());    // TODO unwrap
    }

    pub fn is_greater_or_equal_than_now<'outer_a>(subject_date_time: &'outer_a DateTime) -> bool
    {
        return ChronoDateTimeManipulator::is_greater_or_equal_than(subject_date_time.get_value(), &Utc::now());
    }
}