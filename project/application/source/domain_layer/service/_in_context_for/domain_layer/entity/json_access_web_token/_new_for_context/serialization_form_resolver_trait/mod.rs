use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use std::error::Error;

pub trait SerializationFormResolverTrait {
    type Error: Error;

    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    fn serialize<'a>(
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Result<String, Self::Error>;

    fn deserialize<'a>(
        classic_form: &'a str
    ) -> Result<JsonAccessWebToken<'static>, Self::Error>;
}