pub struct Insert<'a> {
    application_user_email: &'a str,
    value: String,
    wrong_enter_tries_quantity: u8,
    is_approved: bool
}

impl<'a> Insert<'a> {
    pub fn new(
        application_user_email: &'a str,
        value: String,
        wrong_enter_tries_quantity: u8,
        is_approved: bool
    ) -> Self {
        return Self {
            application_user_email,
            value,
            wrong_enter_tries_quantity,
            is_approved
        }
    }

    pub fn into_inner(
        self
    ) -> (&'a str, String, u8, bool) {
        return (
            self.application_user_email,
            self.value,
            self.wrong_enter_tries_quantity,
            self.is_approved
        );
    }
}