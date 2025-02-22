pub struct UserRegistrationToken2 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub is_approved: bool,
    pub expires_at: i64,
}
pub struct UserRegistrationToken3 {
    pub value: String,
    pub is_approved: bool,
    pub expires_at: i64,
    pub can_be_resent_from: i64,
}
