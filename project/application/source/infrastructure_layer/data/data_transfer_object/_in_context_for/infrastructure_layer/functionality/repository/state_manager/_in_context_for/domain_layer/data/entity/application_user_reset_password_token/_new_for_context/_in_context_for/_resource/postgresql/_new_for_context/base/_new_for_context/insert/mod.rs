pub struct Insert {
    application_user_id: i64,
    application_user_reset_password_token_value: String,
    application_user_reset_password_token_wrong_enter_tries_quantity: u8,
    application_user_reset_password_token_is_approved: bool,
}

impl<'a> Insert {
    pub fn new(
        application_user_id: i64,
        application_user_reset_password_token_value: String,
        application_user_reset_password_token_wrong_enter_tries_quantity: u8,
        application_user_reset_password_token_is_approved: bool,
    ) -> Self {
        return Self {
            application_user_id,
            application_user_reset_password_token_value,
            application_user_reset_password_token_wrong_enter_tries_quantity,
            application_user_reset_password_token_is_approved
        }
    }

    pub fn into_inner(
        self
    ) -> (i64, String, u8, bool) {
        return (
            self.application_user_id,
            self.application_user_reset_password_token_value,
            self.application_user_reset_password_token_wrong_enter_tries_quantity,
            self.application_user_reset_password_token_is_approved
        );
    }
}