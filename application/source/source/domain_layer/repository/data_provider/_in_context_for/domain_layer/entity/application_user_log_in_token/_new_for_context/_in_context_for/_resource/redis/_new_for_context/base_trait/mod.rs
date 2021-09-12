use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn get_by_application_user_id_and_device_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection,
        application_user_id: &'outer_b i64,
        device_id: &'outer_b str,
    ) -> Result<Option<ApplicationUserLogInToken<'outer_b>>, Self::Error>;
}