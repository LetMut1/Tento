use {
    super::Encoder,
    crate::infrastructure_layer::data::aggregate_error::AggregateError,
    hmac::{
        Hmac,
        Mac,
    },
    sha3::Sha3_512,
};
pub type HmacSha3_512 = Hmac<Sha3_512>;
impl Encoder<HmacSha3_512> {
    pub fn encode<'a>(salt: &'a [u8], data_for_encode: &'a [u8]) -> Result<Vec<u8>, AggregateError> {
        return Result::Ok(
            Self::prepare(
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
            Self::prepare(
                salt,
                data_for_encode,
            )?
            .verify_slice(encoded_data)
            .is_ok(),
        );
    }
    fn prepare<'a>(salt: &'a [u8], data_for_encode: &'a [u8]) -> Result<HmacSha3_512, AggregateError> {
        let mut hmac = crate::result_return_logic!(HmacSha3_512::new_from_slice(salt));
        hmac.update(data_for_encode);
        return Result::Ok(hmac);
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn validate() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let salt = [0 as u8, 1, 2, 3];
        let data_for_encode = [0 as u8, 1];
        if !Encoder::<HmacSha3_512>::is_valid(
            salt.as_slice(),
            data_for_encode.as_slice(),
            Encoder::<HmacSha3_512>::encode(salt.as_slice(), data_for_encode.as_slice())?.as_slice(),
        )? {
            return Result::Err("".into());
        }
        return Result::Ok(());
    }
}