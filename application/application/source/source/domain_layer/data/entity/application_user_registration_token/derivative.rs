pub struct ApplicationUserRegistrationToken1 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub is_approved: bool,
    pub expires_at: i64,
    pub can_be_resent_from: i64,
}

pub struct ApplicationUserRegistrationToken2 {
    pub can_be_resent_from: i64,
}

pub struct ApplicationUserRegistrationToken3 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub is_approved: bool,
    pub expires_at: i64,
}

pub struct ApplicationUserRegistrationToken4 {
    pub wrong_enter_tries_quantity: i16,
}

pub struct ApplicationUserRegistrationToken5 {
    pub is_approved: bool,
}

pub struct ApplicationUserRegistrationToken6 {
    pub value: String,
    pub is_approved: bool,
    pub expires_at: i64,
    pub can_be_resent_from: i64,
}
