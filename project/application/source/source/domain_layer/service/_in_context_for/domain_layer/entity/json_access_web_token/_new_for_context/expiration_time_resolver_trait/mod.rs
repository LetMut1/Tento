use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use std::error::Error;

pub trait ExpirationTimeResolverTrait {
    type Error: Error;

    fn is_expired<'a>(
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Result<bool, Self::Error>;
}