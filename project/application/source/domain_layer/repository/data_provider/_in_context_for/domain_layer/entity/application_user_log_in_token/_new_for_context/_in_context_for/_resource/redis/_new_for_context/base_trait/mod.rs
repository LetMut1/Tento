use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn find_by_application_user_id_and_device_id<'a, 'b>(
        connection: &'a mut Connection,
        application_user_id: &'b i64,
        device_id: &'b str,
    ) -> Result<Option<ApplicationUserLogInToken<'b>>, Self::Error>;
}