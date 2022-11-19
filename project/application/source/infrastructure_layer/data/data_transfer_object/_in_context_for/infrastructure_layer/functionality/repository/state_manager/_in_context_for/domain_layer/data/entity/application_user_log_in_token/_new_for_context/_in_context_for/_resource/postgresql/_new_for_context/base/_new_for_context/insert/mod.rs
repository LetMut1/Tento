pub struct Insert<'a> {
    application_user_id: i64,
    application_user_log_in_token_device_id: &'a str,
    application_user_log_in_token_value: String,
    application_user_log_in_token_wrong_enter_tries_quantity: u8
}

impl<'a> Insert<'a> {
    pub fn new(
        application_user_id: i64,
        application_user_log_in_token_device_id: &'a str,
        application_user_log_in_token_value: String,
        application_user_log_in_token_wrong_enter_tries_quantity: u8
    ) -> Self {
        return Self {
            application_user_id,
            application_user_log_in_token_device_id,
            application_user_log_in_token_value,
            application_user_log_in_token_wrong_enter_tries_quantity
        }
    }

    pub fn into_inner(
        self
    ) -> (i64, &'a str, String, u8) {
        return (
            self.application_user_id,
            self.application_user_log_in_token_device_id,
            self.application_user_log_in_token_value,
            self.application_user_log_in_token_wrong_enter_tries_quantity
        );
    }
}