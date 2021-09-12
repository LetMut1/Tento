use crate::domain_layer::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn get_by_application_user_id_and_application_user_log_in_token_device_id<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a i64, 
        application_user_log_in_token_device_id: &'outer_a str,
    ) -> Result<Option<JsonRefreshWebToken<'static>>, Self::Error>;

    fn get_by_application_user_id_and_application_user_log_in_token_device_id_registry<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a i64, 
        application_user_log_in_token_device_id_registry: Vec<String>                   // TODO а не &'this Vec<String>   ?
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, Self::Error>;
}