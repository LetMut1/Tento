use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'a>(
        connection: &'a mut Connection, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error>;

    fn delete<'a>(
        connection: &'a mut Connection, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error>;

    fn update_expiration_time<'a>(
        connection: &'a mut Connection,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error>;
}