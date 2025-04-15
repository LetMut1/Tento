use std::marker::PhantomData;
pub struct Value(PhantomData<String>);
impl Value {
    pub const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;
}
pub struct WrongEnterTriesQuantity(PhantomData<u8>);
impl WrongEnterTriesQuantity {
    pub const LIMIT: u8 = 3;
}
pub struct IsApproved(PhantomData<bool>);
pub struct ExpiresAt(PhantomData<i64>);
impl ExpiresAt {
    pub const QUANTITY_OF_MICROSECONDS_FOR_EXPIRATION: i64 = 1000000 * 60 * 30;
}
pub struct CanBeResentFrom(PhantomData<i64>);
impl CanBeResentFrom {
    pub const QUANTITY_OF_MICROSECONDS_BEFORE_RESENDING: i64 = 1000000 * 60;
}
