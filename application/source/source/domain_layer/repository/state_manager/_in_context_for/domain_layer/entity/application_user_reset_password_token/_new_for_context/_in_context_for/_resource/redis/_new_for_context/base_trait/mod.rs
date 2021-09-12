use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error>;

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error>;

    fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error>;
}