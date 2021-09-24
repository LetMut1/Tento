use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_component::payload::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenPayloadFactoryTrait;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;
    type JsonAccessWebTokenPayloadFactory: JsonAccessWebTokenPayloadFactoryTrait<Error = Self::Error>;

    fn create_from_json_refresh_web_token<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<JsonAccessWebToken<'outer_a>, Self::Error> {
        return Ok(
            JsonAccessWebToken::new(
                <Self::JsonAccessWebTokenPayloadFactory as JsonAccessWebTokenPayloadFactoryTrait>::create_from_json_refresh_web_token(json_refresh_web_token)?
            )
        );
    }
}