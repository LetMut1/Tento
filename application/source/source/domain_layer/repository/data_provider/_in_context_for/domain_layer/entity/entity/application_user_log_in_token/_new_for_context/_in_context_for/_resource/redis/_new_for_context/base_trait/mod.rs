use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use redis::Connection;

pub trait BaseTrait {
    fn get_by_application_user_id_and_device_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_b ApplicationUserId, device_id: &'outer_b ApplicationUserLogInTokenDeviceId,
    ) -> Result<Option<ApplicationUserLogInToken<'outer_b>>, BaseError>;
}