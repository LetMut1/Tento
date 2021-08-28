use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use redis::Connection;

#[doc = "Should only be used in crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait implementation."]
pub trait BaseTrait {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;

    fn update<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;


    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError>;
}