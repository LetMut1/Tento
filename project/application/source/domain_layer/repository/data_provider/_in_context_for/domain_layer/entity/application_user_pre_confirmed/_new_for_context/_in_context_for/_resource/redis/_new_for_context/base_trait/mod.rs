use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn is_exist_by_application_user_email<'a>(
        connection: &'a mut Connection,
        application_user_email: &'a str
    ) -> Result<bool, Self::Error>;
}