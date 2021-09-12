use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use std::error::Error;

pub trait SerializationFormResolverTrait {
    type Error: Error;

    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    fn serialize<'outer_a>(
        json_access_web_token: &'outer_a JsonAccessWebToken<'_>
    ) -> Result<String, Self::Error>;

    fn deserialize<'outer_a>(
        classic_form: &'outer_a str
    ) -> Result<JsonAccessWebToken<'static>, Self::Error>;
}