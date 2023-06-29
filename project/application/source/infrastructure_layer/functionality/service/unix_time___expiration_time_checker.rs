use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use super::expiration_time_checker::ExpirationTimeChecker;

pub use crate::infrastructure_layer::data::control_type_registry::UnixTime;

impl ExpirationTimeChecker<UnixTime> {
    pub fn is_expired(date_time: i64) -> bool {
        return !Resolver::<DateTime>::unixtime_is_greater_or_equal_than_now(date_time);
    }
}
