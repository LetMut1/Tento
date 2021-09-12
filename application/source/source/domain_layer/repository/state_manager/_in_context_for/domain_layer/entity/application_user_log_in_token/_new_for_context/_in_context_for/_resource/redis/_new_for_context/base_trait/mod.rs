use crate::domain_layer::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), Self::Error>;

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), Self::Error>;

    fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), Self::Error>;
}