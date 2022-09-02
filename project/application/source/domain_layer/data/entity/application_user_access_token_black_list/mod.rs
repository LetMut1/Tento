pub struct ApplicationUserAccessTokenBlackList<'a> {
    application_user_access_token: &'a str
}

impl<'a> ApplicationUserAccessTokenBlackList<'a> {
    pub fn new(
        application_user_access_token: &'a str
    ) -> Self {
        return Self {
            application_user_access_token
        };
    }

    pub fn get_application_user_access_token<'b>(
        &'b self
    ) -> &'a str {
        return self.application_user_access_token;
    }
}