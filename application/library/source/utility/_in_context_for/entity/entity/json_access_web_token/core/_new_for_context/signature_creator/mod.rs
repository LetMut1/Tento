use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;
use std::env;

pub struct SignatureCreator;

impl SignatureCreator {
    pub fn create<'outer_a>(header_and_payload: &'outer_a str) -> String {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac();
        hmac.input(header_and_payload.as_bytes());

        return hex::encode(hmac.result().code());
    }

    pub fn is_valid<'outer_a>(header_and_payload: &'outer_a str, signature: &'outer_a str) -> bool {
        return Self::create(header_and_payload).as_str() == signature;
    }

    fn get_configured_hmac() -> Hmac<Sha512> {
        return Hmac::new(Sha512::new(), env::var("SECURITY_JAWT_SIGNATURE_ENCODING_PRIVATE_KEY").unwrap().as_bytes());
    }
}