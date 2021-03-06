use crate::entity::core::date_time::DateTime;
use crate::utility::entity::core::date_time::interval_creator::IntervalCreator;

pub struct DateExpirationCreator;

impl<'vague> DateExpirationCreator {
    pub fn create_interval() -> DateTime<'vague> {
        return IntervalCreator::create_from_now(30);
    }
}