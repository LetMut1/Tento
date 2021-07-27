use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub trait SerializationFormResolverTrait {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    fn serialize<'outer_a>(json_access_web_token: &'outer_a JsonAccessWebToken<'_>) -> Result<String, BaseError>;

    fn deserialize<'outer_a>(classic_form: &'outer_a str) -> Result<JsonAccessWebToken<'static>, BaseError>;
}