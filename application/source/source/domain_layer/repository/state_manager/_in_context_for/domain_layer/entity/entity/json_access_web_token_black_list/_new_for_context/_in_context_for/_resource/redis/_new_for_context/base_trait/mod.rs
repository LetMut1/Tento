use crate::domain_layer::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::_component::id::Id as JsonAccessWebTokenId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use redis::Connection;

pub trait BaseTrait {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_access_web_token_black_list: &'outer_a JsonAccessWebTokenBlackList<'_>
    ) -> Result<(), BaseError>;

    fn is_exist_by_json_access_token_id<'outer_a>(
        connection: &'outer_a mut Connection, json_access_web_token_id: &'outer_a JsonAccessWebTokenId
    ) -> Result<bool, BaseError>;
}