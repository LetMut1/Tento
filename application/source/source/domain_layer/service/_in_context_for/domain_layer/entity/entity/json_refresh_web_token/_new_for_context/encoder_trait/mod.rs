use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub trait EncoderTrait {
    fn encode<'outer_a>(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> Result<String, BaseError>;

    fn is_valid<'outer_a>(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>, json_refresh_web_token_hash: &'outer_a str) -> Result<bool, BaseError>;
}