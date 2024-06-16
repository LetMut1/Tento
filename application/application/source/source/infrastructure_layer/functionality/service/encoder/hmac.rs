use super::Encoder;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::error::Error;
use hmac::digest::CtOutput;
use hmac::Hmac;
use hmac::Mac;
use sha3::Sha512;
pub type HmacSha3_512 = Hmac<Sha512>;
impl Encoder<HmacSha3_512> {
    pub fn encode<'a>(salt: &'a [u8], data: &'a [u8]) -> Result<CtOutput<HmacSha3_512>, Auditor<Error>> {
        let hmac = Self::prepare_hmac(salt, data)?;
        let result = hmac.finalize();
        return Ok(result);
    }

    pub fn is_valid<'a>(salt: &'a [u8], data: &'a [u8], encoded_data: &'a [u8]) -> Result<bool, Auditor<Error>> {
        let hmac = Self::prepare_hmac(salt, data)?;
        if let Err(_) = hmac.verify_slice(encoded_data) {
            return Ok(false);
        }
        return Ok(true);
    }

    fn prepare_hmac<'a>(salt: &'a [u8], data: &'a [u8]) -> Result<HmacSha3_512, Auditor<Error>> {
        let mut hmac = HmacSha3_512::new_from_slice(salt).convert(Backtrace::new(line!(), file!()))?;
        hmac.update(data);
        return Ok(hmac);
    }
}
