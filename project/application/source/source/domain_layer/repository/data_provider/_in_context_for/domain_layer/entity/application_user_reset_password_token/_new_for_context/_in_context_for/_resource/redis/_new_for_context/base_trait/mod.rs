use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn find_by_application_user_id<'a, 'b>(
        connection: &'a mut Connection,
        application_user_id: &'b i64
    ) -> Result<Option<ApplicationUserResetPasswordToken<'b>>, Self::Error>;
}