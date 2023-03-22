use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserAccessRefreshToken_UpdatedAtGenerator;

impl ApplicationUserAccessRefreshToken_UpdatedAtGenerator {
    pub fn generate() -> i64 {
        return DateTimeResolver::unixtime_get_now();
    }
}