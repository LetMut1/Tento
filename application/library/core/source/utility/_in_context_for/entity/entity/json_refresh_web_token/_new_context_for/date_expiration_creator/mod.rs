use crate::entity::core::date_time::DateTime;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;

pub struct DateExpirationCreator;

impl DateExpirationCreator {
    pub const QUANTITY_OF_MINUTES: i64 = 60 * 24 * 14;

    pub fn create() -> DateTime {
        return DateTimeManipulator::add_interval_from_now(Self::QUANTITY_OF_MINUTES);
    }
}