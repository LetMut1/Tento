use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn find_by_application_user_pre_confirmed_id<'a, 'b>(
        connection: &'a mut Connection,
        application_user_pre_confirmed_id: &'b i64
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, Self::Error>;
}