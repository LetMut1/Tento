use super::encoder::Encoder;
use hmac::Hmac;
use hmac::Mac;
use hmac::digest::CtOutput;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use sha2::Sha512;

pub type Hmac_Sha2_512 = Hmac<Sha512>;

                                // TODO hex для Стринг.
                                // TODO Sha2-512 or sha3-512 ?
impl Encoder<Hmac_Sha2_512> {
    pub fn encode<'a>(
        salt: &'a [u8],
        data: &'a [u8],
    ) -> Result<CtOutput<Hmac_Sha2_512>, ErrorAuditor_> {
        let hmac = match Self::prepare_hmac(salt, data) {
            Ok(hmac_) => hmac_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let result = hmac.finalize();

        return Ok(result);
    }

    pub fn is_valid<'a>(
        salt: &'a [u8],
        data: &'a [u8],
        encoded_data: &'a [u8],
    ) -> Result<bool, ErrorAuditor_> {
        let hmac = match Self::prepare_hmac(salt, data) {
            Ok(hmac_) => hmac_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        if let Err(_) = hmac.verify_slice(encoded_data) {
            return Ok(false);
        }

        return Ok(true);
    }

    fn prepare_hmac<'a>(
        salt: &'a [u8],
        data: &'a [u8],
    ) -> Result<Hmac_Sha2_512, ErrorAuditor_> {
        let mut hmac = match Hmac_Sha2_512::new_from_slice(salt) {
            Ok(hmac_) => hmac_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        hmac.update(data);

        return Ok(hmac);
    }
}