use crate::domain_layer::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use redis::Connection;
use std::error::Error;

#[doc = "Should only be used in crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait implementation."]
pub trait BaseTrait {
    type Error: Error;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error>;

    fn update<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error>;


    fn delete<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error>;
}