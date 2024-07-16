use super::Encoder;
use crate::infrastructure_layer::data::{
    alternative_workflow::{
        AlternativeWorkflow,
        ResultConverter,
    },
    auditor::Backtrace,
};
use hmac::{
    digest::CtOutput,
    Hmac,
    Mac,
};
use sha3::Sha512;
pub type HmacSha3_512 = Hmac<Sha512>;
impl Encoder<HmacSha3_512> {
    pub fn encode<'a>(salt: &'a [u8], data: &'a [u8]) -> Result<CtOutput<HmacSha3_512>, AlternativeWorkflow> {
        return Ok(
            Self::prepare_hmac(
                salt,
                data,
            )?
            .finalize(),
        );
    }
    pub fn is_valid<'a>(salt: &'a [u8], data: &'a [u8], encoded_data: &'a [u8]) -> Result<bool, AlternativeWorkflow> {
        return Ok(
            Self::prepare_hmac(
                salt,
                data,
            )?
            .verify_slice(encoded_data)
            .is_ok(),
        );
    }
    fn prepare_hmac<'a>(salt: &'a [u8], data: &'a [u8]) -> Result<HmacSha3_512, AlternativeWorkflow> {
        let mut hmac = HmacSha3_512::new_from_slice(salt).into_internal_error_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        hmac.update(data);
        return Ok(hmac);
    }
}
