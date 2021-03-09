use chrono::Duration;
use chrono::offset::Utc;
use crate::entity::core::date_time::DateTime;
use crate::utility::chrono_date_time_manipulator::ChronoDateTimeManipulator;
use maybe_owned::MaybeOwned;

pub struct DateTimeManipulator;

impl<'outer, 'vague> DateTimeManipulator {
    // pub fn add_interval(date_time: &'outer DateTime<'outer>, quantity_of_minutes: i64) -> () {              // TODO написать тесты. проверить, создатся ли интервал. Сделегировать на utility_chrono_date_ime_manipulator?
    //     date_time.get_value().checked_add_signed(Duration::minutes(quantity_of_minutes)).unwrap();  // TODO unwrap

    //     return ();
    // }

    pub fn add_interval_from_now(quantity_of_minutes: i64) -> DateTime<'vague> {
        return DateTime::new_from_date_time(
            MaybeOwned::Owned(Utc::now().checked_add_signed(Duration::minutes(quantity_of_minutes)).unwrap())   // TODO unwrap
        );
    }

    pub fn is_greate_or_equal_than_now(subject_date_time: &'outer DateTime) -> bool
    {
        return ChronoDateTimeManipulator::is_greate_or_equal_than(subject_date_time.get_value(), &Utc::now());
    }
}