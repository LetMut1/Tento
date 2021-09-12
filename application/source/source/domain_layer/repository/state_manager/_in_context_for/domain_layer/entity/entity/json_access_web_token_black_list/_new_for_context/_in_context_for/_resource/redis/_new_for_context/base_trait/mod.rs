use crate::domain_layer::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        json_access_web_token_black_list: &'outer_a JsonAccessWebTokenBlackList<'_>
    ) -> Result<(), Self::Error>;
}