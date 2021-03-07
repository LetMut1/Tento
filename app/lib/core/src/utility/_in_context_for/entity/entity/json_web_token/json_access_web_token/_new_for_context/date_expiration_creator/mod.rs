use crate::entity::core::date_time::DateTime;
use crate::utility::_in_context_for::entity::core::date_time::_new_for_context::interval_creator::IntervalCreator;

pub struct DateExpirationCreator;

impl<'vague> DateExpirationCreator {
    pub fn create_interval() -> DateTime<'vague> {
        return IntervalCreator::create_from_now(30);
    }
}