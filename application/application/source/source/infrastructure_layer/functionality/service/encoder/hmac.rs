use super::Encoder;
use hmac::Hmac;
use hmac::Mac;
use hmac::digest::CtOutput;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::data::error::Runtime;
use sha3::Sha512;

pub type Hmac_Sha3_512 = Hmac<Sha512>;

impl Encoder<Hmac_Sha3_512> {
    pub fn encode<'a>(
        salt: &'a [u8],
        data: &'a [u8],
    ) -> Result<CtOutput<Hmac_Sha3_512>, Auditor<Error>> {
        let hmac = match Self::prepare_hmac(salt, data) {
            Ok(hmac_) => hmac_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
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
    ) -> Result<bool, Auditor<Error>> {
        let hmac = match Self::prepare_hmac(salt, data) {
            Ok(hmac_) => hmac_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
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
    ) -> Result<Hmac_Sha3_512, Auditor<Error>> {
        let mut hmac = match Hmac_Sha3_512::new_from_slice(salt) {
            Ok(hmac_) => hmac_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Runtime::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        hmac.update(data);

        return Ok(hmac);
    }
}