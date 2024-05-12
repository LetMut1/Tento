use super::Encoder;
use hmac::Hmac;
use hmac::Mac;
use hmac::digest::CtOutput;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Converter;
use sha3::Sha512;

pub type HmacSha3512 = Hmac<Sha512>;

impl Encoder<HmacSha3512> {
    pub fn encode<'a>(
        salt: &'a [u8],
        data: &'a [u8],
    ) -> Result<CtOutput<HmacSha3512>, Auditor<Error>> {
        let hmac = Self::prepare_hmac(salt, data)?;

        let result = hmac.finalize();

        return Ok(result);
    }

    pub fn is_valid<'a>(
        salt: &'a [u8],
        data: &'a [u8],
        encoded_data: &'a [u8],
    ) -> Result<bool, Auditor<Error>> {
        let hmac = Self::prepare_hmac(salt, data)?;

        if let Err(_) = hmac.verify_slice(encoded_data) {
            return Ok(false);
        }

        return Ok(true);
    }

    fn prepare_hmac<'a>(
        salt: &'a [u8],
        data: &'a [u8],
    ) -> Result<HmacSha3512, Auditor<Error>> {
        let mut hmac = HmacSha3512::new_from_slice(salt).convert(BacktracePart::new(line!(), file!()))?;

        hmac.update(data);

        return Ok(hmac);
    }
}