use super::Encoder;
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use hmac::{
    Hmac,
    Mac,
};
use sha3::Sha3_512;
pub type HmacSha3_512 = Hmac<Sha3_512>;
impl Encoder<HmacSha3_512> {
    pub fn encode<'a>(salt: &'a [u8], data_for_encode: &'a [u8]) -> Result<Vec<u8>, AggregateError> {
        return Result::Ok(
            Self::prepare_hmac(
                salt,
                data_for_encode,
            )?
            .finalize()
            .into_bytes()
            .to_vec(),
        );
    }
    pub fn is_valid<'a>(salt: &'a [u8], data_for_encode: &'a [u8], encoded_data: &'a [u8]) -> Result<bool, AggregateError> {
        return Result::Ok(
            Self::prepare_hmac(
                salt,
                data_for_encode,
            )?
            .verify_slice(encoded_data)
            .is_ok(),
        );
    }
    fn prepare_hmac<'a>(salt: &'a [u8], data_for_encode: &'a [u8]) -> Result<HmacSha3_512, AggregateError> {
        let mut hmac = HmacSha3_512::new_from_slice(salt).into_logic(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        hmac.update(data_for_encode);
        return Result::Ok(hmac);
    }
}
