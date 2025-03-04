use std::marker::PhantomData;
pub struct ExpiresAt(PhantomData<i64>);
impl ExpiresAt {
    pub const QUANTITY_OF_SECONDS_FOR_EXPIRATION: i64 = 60 * 60 * 24;
}
