use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use redis::Connection;
use std::error::Error;

pub trait RepositoryProxyTrait {
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

    fn get_by_application_user_id<'a>(
        connection: &'a mut Connection,
        application_user_id: &'a i64
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, Self::Error>;
}