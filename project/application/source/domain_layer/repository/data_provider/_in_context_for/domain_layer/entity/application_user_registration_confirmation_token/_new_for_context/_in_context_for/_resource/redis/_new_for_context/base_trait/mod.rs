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

    fn find_by_application_user_emailXXXxDelete<'a, 'b>(
        connection: &'a mut Con,
        application_user_email: &'b str
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, Self::Error>;
}

pub trait BaseTrait {
    type Error: Error;

    fn find_by_application_user_email<'a, 'b>(
        connection: &'a mut Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>,
        application_user_email: &'b str
    ) -> Pin<Box<dyn Future<Output = Result<Option<ApplicationUserRegistrationConfirmationToken<'b>>, Self::Error>> + Send + 'a>>
    where
        'b: 'a;
}