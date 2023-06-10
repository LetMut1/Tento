use extern_crate::crypto::hmac::Hmac as Hmac_;
use extern_crate::crypto::mac::Mac;
use extern_crate::crypto::sha2::Sha512;
use super::encoder::Encoder;

pub use crate::infrastructure_layer::data::control_type_registry::Hmac;

impl Encoder<Hmac> {        // TODO https://docs.rs/hmac/latest/hmac/ - этот hmac может верифицировать.
    pub fn encode<'a>(salt: &'a [u8], data: &'a [u8], encoded_data: &'a mut [u8]) -> () {
        let mut hmac = Hmac_::new(Sha512::new(), salt);

        hmac.input(data);

        hmac.raw_result(encoded_data);  // TODO TIme attack

        return ();
    }
}