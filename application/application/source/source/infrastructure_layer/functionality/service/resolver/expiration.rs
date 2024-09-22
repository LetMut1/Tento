use super::{
    date_time::UnixTime,
    Resolver,
};
pub struct Expiration;
impl Resolver<Expiration> {
    pub fn is_expired(date_time: i64) -> bool {
        return !Resolver::<UnixTime>::is_greater_or_equal_than_now(date_time);
    }
}
