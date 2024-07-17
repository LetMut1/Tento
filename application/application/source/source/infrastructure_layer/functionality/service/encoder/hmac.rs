use super::Encoder;
use crate::infrastructure_layer::data::aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use hmac::{
    digest::CtOutput,
    Hmac,
    Mac,
};
use sha3::Sha512;
pub type HmacSha3_512 = Hmac<Sha512>;
impl Encoder<HmacSha3_512> {
    pub fn encode<'a>(salt: &'a [u8], data: &'a [u8]) -> Result<CtOutput<HmacSha3_512>, AggregateError> {
        return Ok(
            Self::prepare_hmac(
                salt,
                data,
            )?
            .finalize(),
        );
    }
    pub fn is_valid<'a>(salt: &'a [u8], data: &'a [u8], encoded_data: &'a [u8]) -> Result<bool, AggregateError> {
        return Ok(
            Self::prepare_hmac(
                salt,
                data,
            )?
            .verify_slice(encoded_data)
            .is_ok(),
        );
    }
    fn prepare_hmac<'a>(salt: &'a [u8], data: &'a [u8]) -> Result<HmacSha3_512, AggregateError> {
        let mut hmac = HmacSha3_512::new_from_slice(salt).into_invalid_argument_from_client_code(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        hmac.update(data);
        return Ok(hmac);
    }
}
