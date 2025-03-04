use std::marker::PhantomData;
pub struct ObfuscationValue(PhantomData<String>);
pub struct ExpiresAt(PhantomData<i64>);
impl ExpiresAt {
    pub const QUANTITY_OF_SECONDS_FOR_EXPIRATION: i64 = 60 * 60 * 24 * 30 * 3;
}
pub struct UpdatedAt(PhantomData<i64>);
