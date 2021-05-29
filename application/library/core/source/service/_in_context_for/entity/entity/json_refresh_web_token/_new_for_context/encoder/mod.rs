use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::data_transfer_object::_in_context_for::entity::entity::json_refresh_web_token::_new_fro_context::common::Common;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;
use serde_json;

pub struct Encoder;

impl Encoder {
    const PRIVATE_KEY: &'static str = "kC2a1mXFi3sc9gE2udB0qL02gzJd2asu4S1ksMsJp12v8cs5fFm6dV2wq";  // TODO где должен быть ключ ( в енв)

    pub fn encode<'outer_a>(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> String {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac();
        
        hmac.input(serde_json::to_string(&Common::new(json_refresh_web_token)).unwrap().as_bytes());

        return hex::encode(hmac.result().code());
    }

    pub fn is_valid<'outer_a>(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>, json_refresh_web_token_hash: &'outer_a str) -> bool {
        return Self::encode(json_refresh_web_token).as_str() == json_refresh_web_token_hash;
    }

    fn get_configured_hmac() -> Hmac<Sha512> {
        return Hmac::new(Sha512::new(), Self::PRIVATE_KEY.as_bytes());
    }
}