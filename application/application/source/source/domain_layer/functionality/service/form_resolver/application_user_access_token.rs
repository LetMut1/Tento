use super::FormResolver;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::encoder::base64::Base64;
use crate::infrastructure_layer::functionality::service::encoder::Encoder as Encoder_;
use crate::infrastructure_layer::functionality::service::encoder::hmac::Hmac_Sha3_512;
use crate::infrastructure_layer::functionality::service::serializer::MessagePack;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;

impl FormResolver<ApplicationUserAccessToken<'_>> {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    pub fn to_encrypted<'a>(application_user_access_token: &'a ApplicationUserAccessToken<'_>) -> Result<ApplicationUserAccessTokenEncrypted, ErrorAuditor_> {
        let data = match Serializer::<MessagePack>::serialize(application_user_access_token) {
            Ok(data_) => data_,
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

        let application_user_access_token_serialized = Encoder_::<Base64>::encode(data.as_slice());

        let application_user_access_token_serialized_signature = match Encoder::<Signature>::encode(application_user_access_token_serialized.as_bytes()) {
            Ok(application_user_access_token_serialized_signature_) => application_user_access_token_serialized_signature_,
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

        let application_user_access_token_encrypted = format!(
            "{}{}{}",
            application_user_access_token_serialized.as_str(),
            Self::TOKEN_PARTS_SEPARATOR,
            application_user_access_token_serialized_signature.as_str()
        );

        return Ok(
            ApplicationUserAccessTokenEncrypted(application_user_access_token_encrypted)
        );
    }

    pub fn from_encrypted<'a>(application_user_access_token_encrypted: &'a ApplicationUserAccessTokenEncrypted) -> Result<InvalidArgumentResult<ApplicationUserAccessToken<'static>>, ErrorAuditor_> {
        let mut token_part_registry = application_user_access_token_encrypted.0.as_str().splitn::<'_, &'_ str>(
            2,
            Self::TOKEN_PARTS_SEPARATOR,
        );

        let application_user_access_token_serialized = match token_part_registry.next() {
            Some(application_user_access_token_serialized_) => application_user_access_token_serialized_,
            None => {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument: InvalidArgument::ApplicationUserAccessTokenEncrypted,
                    }
                );
            }
        };

        let application_user_access_token_serialized_signature = match token_part_registry.next() {
            Some(application_user_access_token_serialized_signature_) => application_user_access_token_serialized_signature_,
            None => {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument: InvalidArgument::ApplicationUserAccessTokenEncrypted,
                    }
                );
            }
        };

        let is_valid = match Encoder::<Signature>::is_valid(
            application_user_access_token_serialized.as_bytes(),
            application_user_access_token_serialized_signature.as_bytes(),
        )
        {
            Ok(is_valid_) => is_valid_,
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

        if !is_valid {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserAccessTokenEncrypted,
                }
            );
        }

        let data = match Encoder_::<Base64>::decode(application_user_access_token_serialized.as_bytes()) {
            Ok(data_) => data_,
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

        let application_user_access_token = match Serializer::<MessagePack>::deserialize::<'_, ApplicationUserAccessToken<'static>>(data.as_slice()) {
            Ok(application_user_access_token_) => application_user_access_token_,
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

        return Ok(
            InvalidArgumentResult::Ok {
                subject: application_user_access_token,
            },
        );
    }
}

struct Signature;

impl Encoder<Signature> {
    fn encode<'a>(application_user_access_token_serialized: &'a [u8]) -> Result<String, ErrorAuditor_> {
        let application_user_access_token_serialized_encoded = match Encoder_::<Hmac_Sha3_512>::encode(
            ENVIRONMENT_CONFIGURATION.encryption.private_key.application_user_access_token.0.as_bytes(),
            application_user_access_token_serialized,
        )
        {
            Ok(application_user_access_token_serialized_encoded_) => application_user_access_token_serialized_encoded_,
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

        let application_user_access_token_serialized_signature = Encoder_::<Base64>::encode(application_user_access_token_serialized_encoded.into_bytes().as_slice());

        return Ok(application_user_access_token_serialized_signature);
    }

    fn is_valid<'a>(
        application_user_access_token_serialized: &'a [u8],
        application_user_access_token_serialized_signature: &'a [u8],
    ) -> Result<bool, ErrorAuditor_> {
        let application_user_access_token_serialized_encoded = match Encoder_::<Base64>::decode(application_user_access_token_serialized_signature) {
            Ok(application_user_access_token_serialized_encoded_) => application_user_access_token_serialized_encoded_,
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

        let is_valid = match Encoder_::<Hmac_Sha3_512>::is_valid(
            ENVIRONMENT_CONFIGURATION.encryption.private_key.application_user_access_token.0.as_bytes(),
            application_user_access_token_serialized,
            application_user_access_token_serialized_encoded.as_slice(),
        )
        {
            Ok(is_valid_) => is_valid_,
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

        return Ok(is_valid);
    }
}
