use crate::domain_layer::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use redis::Connection;

pub trait BaseTrait {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), BaseError>;

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), BaseError>;

    fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), BaseError>;
}