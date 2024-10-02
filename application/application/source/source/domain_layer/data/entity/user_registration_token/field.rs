pub struct Value;
impl Value {
    pub const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;
}
pub struct WrongEnterTriesQuantity;
impl WrongEnterTriesQuantity {
    pub const LIMIT: i16 = 5;
}
pub struct IsApproved;
pub struct ExpiresAt;
impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 60 * 3;
}
pub struct CanBeResentFrom;
impl CanBeResentFrom {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
}
