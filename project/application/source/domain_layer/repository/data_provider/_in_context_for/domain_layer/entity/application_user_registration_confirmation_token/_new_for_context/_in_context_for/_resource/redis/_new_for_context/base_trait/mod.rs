use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn find_by_application_user_email<'a, 'b>(
        connection: &'a mut Connection,
        application_user_email: &'b str
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, Self::Error>;
}