use super::{
    date_time::DateTime,
    Resolver
};
pub struct Expiration;
impl Resolver<Expiration> {
    pub fn is_expired(date_time: i64) -> bool {
        return !Resolver::<DateTime>::unixtime_is_greater_or_equal_than_now(date_time);
    }
}
