use crate::domain_layer::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn get_by_application_user_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection,
        application_user_id: &'outer_b i64
    ) -> Result<Option<ApplicationUserResetPasswordToken<'outer_b>>, Self::Error>;
}