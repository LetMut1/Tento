use chrono::Duration;
use chrono::offset::Utc;
use crate::entity::core::date_time::DateTime;
use maybe_owned::MaybeOwned;

pub struct IntervalCreator;

impl<'vague> IntervalCreator {
    pub fn create_from_now(quantity_of_minutes: i64) -> DateTime<'vague> {
        return DateTime::new_from_date_time(
            MaybeOwned::Owned(Utc::now().checked_add_signed(Duration::minutes(quantity_of_minutes)).unwrap())
        );
    }
}