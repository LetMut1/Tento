use std::marker::PhantomData;
pub struct Id(PhantomData<String>);
pub struct ExpiresAt(PhantomData<i64>);
impl ExpiresAt {
    pub const QUANTITY_OF_MICROSECONDS_FOR_EXPIRATION: i64 = 1000000 * 60 * 60;
}
