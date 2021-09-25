use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use redis::Connection;
use std::error::Error;

#[doc = "Should only be used in crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait implementation."]
pub trait BaseTrait {
    type Error: Error;

    fn create<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error>;

    fn update<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error>;


    fn delete<'a>(
        connection: &'a mut Connection,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error>;
}