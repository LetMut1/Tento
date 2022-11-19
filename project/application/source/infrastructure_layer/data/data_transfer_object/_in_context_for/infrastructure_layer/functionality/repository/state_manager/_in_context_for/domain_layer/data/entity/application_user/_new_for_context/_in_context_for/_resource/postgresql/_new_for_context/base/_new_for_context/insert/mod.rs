pub struct Insert {
    application_user_email: String,
    application_user_nickname: String,
    application_user_password_hash: String
}

impl Insert {
    pub fn new(
        application_user_email: String,
        application_user_nickname: String,
        application_user_password_hash: String
    ) -> Self {
        return Self {
            application_user_email,
            application_user_nickname,
            application_user_password_hash
        }
    }

    pub fn into_inner(
        self
    ) -> (String, String, String) {
        return (
            self.application_user_email,
            self.application_user_nickname,
            self.application_user_password_hash
        );
    }
}