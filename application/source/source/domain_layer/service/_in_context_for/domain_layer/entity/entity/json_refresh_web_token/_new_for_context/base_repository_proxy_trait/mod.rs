use crate::domain_layer::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::error::base_error::base_error::BaseError;
use redis::Connection;

pub trait BaseRepositoryProxyTrait {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;

    fn update<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;

    fn get_by_application_user_id_and_application_user_log_in_token_device_id<'outer_a, 'vague>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId
    ) -> Result<Option<JsonRefreshWebToken<'vague>>, BaseError>;

    fn get_by_application_user_id<'outer_a, 'vague>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_a ApplicationUserId
    ) -> Result<Option<Vec<JsonRefreshWebToken<'vague>>>, BaseError>;
}