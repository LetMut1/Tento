pub struct UserAuthorizationToken2 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub expires_at: i64,
}
pub struct UserAuthorizationToken3 {
    pub value: String,
    pub expires_at: i64,
    pub can_be_resent_from: i64,
}
