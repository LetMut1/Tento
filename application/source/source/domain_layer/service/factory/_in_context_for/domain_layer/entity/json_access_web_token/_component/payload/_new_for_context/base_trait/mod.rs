use crate::domain_layer::entity::json_access_web_token::_component::payload::Payload;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;

    fn create_from_json_refresh_web_token<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<Payload<'outer_a>, Self::Error>;
}