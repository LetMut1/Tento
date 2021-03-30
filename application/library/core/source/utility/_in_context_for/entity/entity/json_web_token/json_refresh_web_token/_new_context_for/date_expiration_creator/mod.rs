use crate::entity::core::date_time::DateTime;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;

pub struct DateExpirationCreator;

impl DateExpirationCreator {
    pub fn create() -> DateTime {
        return DateTimeManipulator::add_interval_from_now(60 * 24 * 14);
    }
}