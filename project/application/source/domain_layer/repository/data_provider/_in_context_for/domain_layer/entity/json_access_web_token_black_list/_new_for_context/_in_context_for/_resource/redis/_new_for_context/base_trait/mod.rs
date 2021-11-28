use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn is_exist_by_json_access_token_id<'a>(
        connection: &'a mut Connection,
        json_access_web_token_id: &'a str
    ) -> Result<bool, Self::Error>;
}