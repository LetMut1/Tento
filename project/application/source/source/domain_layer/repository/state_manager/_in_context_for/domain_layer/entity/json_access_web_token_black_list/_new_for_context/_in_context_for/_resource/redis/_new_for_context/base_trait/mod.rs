use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'a>(
        connection: &'a mut Connection,
        json_access_web_token_black_list: &'a JsonAccessWebTokenBlackList<'_>
    ) -> Result<(), Self::Error>;
}