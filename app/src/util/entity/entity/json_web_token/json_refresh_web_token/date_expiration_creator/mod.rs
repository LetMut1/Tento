use chrono::Duration;
use chrono::offset::Utc;
use crate::entity::core::date_time::DateTime;
use maybe_owned::MaybeOwned;

pub struct DateExpirationCreator;

impl<'a> DateExpirationCreator {
    pub fn create_interval() -> DateTime<'a> {
        return DateTime::new_from_date_time(MaybeOwned::Owned(Utc::now().checked_add_signed(Duration::days(30)).unwrap()));
    }
}