use chrono::Duration;
use chrono::offset::Utc;
use crate::domain_layer::entity::proxed_type::date_time::DateTime;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::chrono_date_time_manipulator::ChronoDateTimeManipulator;

pub struct DateTimeManipulator;

impl DateTimeManipulator {
    // pub fn add_interval<'outer_a>(date_time: &'outer_a DateTime, quantity_of_minutes: i64) -> () {              // TODO написать тесты. проверить, создатся ли интервал.
    //     date_time.get_value().checked_add_signed(Duration::minutes(quantity_of_minutes)).unwrp();

    //     return ();
    // }

    pub fn add_interval_from_now(quantity_of_minutes: i64) -> Result<DateTime, BaseError> {
        match Utc::now().checked_add_signed(Duration::minutes(quantity_of_minutes)) {
            Some(chrono_date_time) => {
                return Ok(DateTime::new_from_date_time(chrono_date_time));
            },
            None => {
                return Err(BaseError::LogicError("Too big date must not be added."));
            }
        };
    }

    pub fn is_greater_or_equal_than_now<'outer_a>(subject_date_time: &'outer_a DateTime) -> bool
    {
        return ChronoDateTimeManipulator::is_greater_or_equal_than(subject_date_time.get_value(), &Utc::now());
    }
}