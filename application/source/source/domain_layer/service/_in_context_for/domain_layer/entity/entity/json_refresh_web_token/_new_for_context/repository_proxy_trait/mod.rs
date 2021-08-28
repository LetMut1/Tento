use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use redis::Connection;

pub trait RepositoryProxyTrait {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;

    fn update<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;

    fn get_by_application_user_id<'outer_a>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_a ApplicationUserId
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, BaseError>;
}