use super::encoder::Encoder;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user::ApplicationUser_PasswordHash;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::encoder::Argon2Id;
use crate::infrastructure_layer::functionality::service::encoder::Encoder as Encoder_;

impl Encoder<ApplicationUser_Password> {
    pub fn encode<'a>(application_user_password: &'a ApplicationUser_Password) -> Result<ApplicationUser_PasswordHash, ErrorAuditor> {
        let application_user_password_hash = match Encoder_::<Argon2Id>::encode(application_user_password.get().as_bytes()) {
            Ok(application_user_password_hash_) => application_user_password_hash_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
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

        return Ok(ApplicationUser_PasswordHash::new(application_user_password_hash));
    }

    pub fn is_valid<'a>(
        application_user_password: &'a ApplicationUser_Password,
        application_user_password_hash: &'a ApplicationUser_PasswordHash,
    ) -> Result<bool, ErrorAuditor> {
        let value = match Encoder_::<Argon2Id>::is_valid(
            application_user_password.get().as_bytes(),
            application_user_password_hash.get(),
        ) {
            Ok(value_) => value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::OtherError {
                                other_error: OtherError::new(error),
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

        return Ok(value);
    }
}
