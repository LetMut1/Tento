use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use redis::Connection;
use std::error::Error;

pub trait RepositoryProxyTrait {
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

    fn get_by_application_user_id<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_id: &'outer_a i64
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, Self::Error>;
}