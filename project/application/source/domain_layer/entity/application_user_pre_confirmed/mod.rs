pub struct ApplicationUserPreConfirmed {
    application_user_email: String
}

impl ApplicationUserPreConfirmed {
    pub fn new(
        application_user_email: String
    ) -> Self {
        return Self {
            application_user_email
        };
    }

    pub fn get_application_user_email<'a>(
        &'a self
    ) -> &'a str {
        return self.application_user_email.as_str();
    }

    pub fn into_inner(
        self
    ) -> String {
        return self.application_user_email;
    }
}