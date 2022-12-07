/// This entity is not used yet, and we need to decide whether we will use this flow.
///
/// Tokens are refreshed every N minutes. That is, N minutes the token remains
/// valid from the point of view of the system that created it, that is, it can be used.
/// When a user logs out, he stops using the token, but it can still be used for a while.
/// Therefore, those tokens that we delete for the user must be recorded in this entity.
/// Wherever the validity of a token is checked, you need to check if this token is in this
/// entity, and block the action if it is.
pub struct ApplicationUserAccessTokenBlackList<'a> {
    application_user_access_token_id: &'a str
}

impl<'a> ApplicationUserAccessTokenBlackList<'a> {
    pub fn new(
        application_user_access_token_id: &'a str
    ) -> Self {
        return Self {
            application_user_access_token_id
        };
    }

    pub fn get_application_user_access_token_id<'b>(
        &'b self
    ) -> &'a str {
        return self.application_user_access_token_id;
    }
}