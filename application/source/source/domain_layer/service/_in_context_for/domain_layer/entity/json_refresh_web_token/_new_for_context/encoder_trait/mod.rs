use crate::domain_layer::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use std::error::Error;

pub trait EncoderTrait {
    type Error: Error;

    fn encode<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<String, Self::Error>;

    fn is_valid<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>,
        json_refresh_web_token_hash: &'outer_a str
    ) -> Result<bool, Self::Error>;
}