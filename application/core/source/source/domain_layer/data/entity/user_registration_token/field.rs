use std::marker::PhantomData;
pub struct Value(PhantomData<String>);
impl Value {
    pub const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;
}
pub struct WrongEnterTriesQuantity(PhantomData<i16>);
impl WrongEnterTriesQuantity {
    pub const LIMIT: i16 = 5;
}
pub struct IsApproved(PhantomData<bool>);
pub struct ExpiresAt(PhantomData<i64>);
impl ExpiresAt {
    pub const QUANTITY_OF_SECONDS_FOR_EXPIRATION: i64 = 60 * 60 * 24;
}
pub struct CanBeResentFrom(PhantomData<i64>);
impl CanBeResentFrom {
    pub const QUANTITY_OF_SECONDS_BEFORE_RESENDING: i64 = 60;
}
