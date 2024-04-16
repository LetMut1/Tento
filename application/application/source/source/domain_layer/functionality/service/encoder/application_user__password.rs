use super::Encoder;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user::ApplicationUser_PasswordHash;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::functionality::service::encoder::argon2id::Argon2Id;
use crate::infrastructure_layer::functionality::service::encoder::Encoder as Encoder_;

impl Encoder<ApplicationUser_Password> {
    pub fn encode<'a>(application_user_password: &'a ApplicationUser_Password) -> Result<ApplicationUser_PasswordHash, Auditor<Error>> {
        let application_user_password_hash = match Encoder_::<Argon2Id>::encode(application_user_password.0.as_bytes()) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
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

        return Ok(ApplicationUser_PasswordHash(application_user_password_hash));
    }

    pub fn is_valid<'a>(
        application_user_password: &'a ApplicationUser_Password,
        application_user_password_hash: &'a ApplicationUser_PasswordHash,
    ) -> Result<bool, Auditor<Error>> {
        let value = match Encoder_::<Argon2Id>::is_valid(
            application_user_password.0.as_bytes(),
            application_user_password_hash.0.as_str(),
        ) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
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

        return Ok(value);
    }
}
