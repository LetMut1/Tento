use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use redis::Connection;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn find_by_application_user_id_and_application_user_log_in_token_device_id<'a>(
        connection: &'a mut Connection, 
        application_user_id: &'a i64, 
        application_user_log_in_token_device_id: &'a str,
    ) -> Result<Option<JsonRefreshWebToken<'static>>, Self::Error>;

    fn find_by_application_user_id_and_application_user_log_in_token_device_id_registry<'a>(
        connection: &'a mut Connection, 
        application_user_id: &'a i64, 
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, Self::Error>;
}