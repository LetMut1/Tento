use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn get_by_application_user_pre_confirmed_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection,
        application_user_pre_confirmed_id: &'outer_b i64
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'outer_b>>, Self::Error>;
}