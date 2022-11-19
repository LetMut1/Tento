pub struct Insert<'a> {
    application_user_email: &'a str,
    application_user_registration_confirmation_token_value: String,
    application_user_registration_confirmation_token_wrong_enter_tries_quantity: u8,
    application_user_registration_confirmation_token_is_approved: bool
}

impl<'a> Insert<'a> {
    pub fn new(
        application_user_email: &'a str,
        application_user_registration_confirmation_token_value: String,
        application_user_registration_confirmation_token_wrong_enter_tries_quantity: u8,
        application_user_registration_confirmation_token_is_approved: bool
    ) -> Self {
        return Self {
            application_user_email,
            application_user_registration_confirmation_token_value,
            application_user_registration_confirmation_token_wrong_enter_tries_quantity,
            application_user_registration_confirmation_token_is_approved
        }
    }

    pub fn into_inner(
        self
    ) -> (&'a str, String, u8, bool) {
        return (
            self.application_user_email,
            self.application_user_registration_confirmation_token_value,
            self.application_user_registration_confirmation_token_wrong_enter_tries_quantity,
            self.application_user_registration_confirmation_token_is_approved
        );
    }
}