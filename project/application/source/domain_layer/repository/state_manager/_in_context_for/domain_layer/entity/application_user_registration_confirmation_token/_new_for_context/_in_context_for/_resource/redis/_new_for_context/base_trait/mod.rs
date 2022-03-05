use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use redis_ref::aio::AsyncStream;
use redis_ref::aio::Connection;
use redis::Connection as Con;
use std::boxed::Box;
use std::error::Error;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use std::pin::Pin;

pub trait BaseTraitXXXxDelete {
    type Error: Error;

    fn createXXXxDelete<'a>(
        connection: &'a mut Con, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error>;

    fn deleteXXXxDelete<'a>(
        connection: &'a mut Con, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error>;

    fn update_expiration_timeXXXxDelete<'a>(
        connection: &'a mut Con,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error>;
}

pub trait BaseTrait {
    type Error: Error;

    fn create<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;

    fn delete<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>, 
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;

    fn update_expiration_time<'a>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;
}