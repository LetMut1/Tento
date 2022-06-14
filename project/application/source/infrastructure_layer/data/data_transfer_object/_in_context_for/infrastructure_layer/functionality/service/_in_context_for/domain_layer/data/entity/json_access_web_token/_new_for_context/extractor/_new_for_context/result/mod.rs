use crate::domain_layer::data::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;

pub enum Result {
    JsonAccessWebToken {
        json_access_web_token: JsonAccessWebToken<'static>
    },
    JsonAccessWebTokenAlreadyExpired,
    JsonAccessWebTokenInJsonAccessWebTokenBlackList
}