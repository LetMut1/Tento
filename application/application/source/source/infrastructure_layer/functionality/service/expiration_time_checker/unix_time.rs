use super::ExpirationTimeChecker;
use crate::infrastructure_layer::{
    functionality::service::resolver::Resolver,
};
pub struct UnixTime;
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
impl ExpirationTimeChecker<UnixTime> {
    pub fn is_expired(date_time: i64) -> bool {
        return !Resolver::<DateTime>::unixtime_is_greater_or_equal_than_now(date_time);
    }
}
