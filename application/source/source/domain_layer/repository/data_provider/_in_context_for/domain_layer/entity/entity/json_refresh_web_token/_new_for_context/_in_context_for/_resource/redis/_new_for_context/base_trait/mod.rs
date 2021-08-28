use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use redis::Connection;

pub trait BaseTrait {
    fn get_by_application_user_id_and_application_user_log_in_token_device_id<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId,
    ) -> Result<Option<JsonRefreshWebToken<'static>>, BaseError>;

    fn get_by_application_user_id_and_application_user_log_in_token_device_id_registry<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, BaseError>;
}