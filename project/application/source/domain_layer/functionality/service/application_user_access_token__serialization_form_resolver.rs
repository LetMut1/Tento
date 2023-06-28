use crate::domain_layer::data::entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::pushable_environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::functionality::service::encoder::Base64;
use crate::infrastructure_layer::functionality::service::encoder::Encoder as Encoder_;
use crate::infrastructure_layer::functionality::service::encoder::Hmac;
use crate::infrastructure_layer::functionality::service::serializer::MessagePack;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use super::serialization_form_resolver::SerializationFormResolver;

impl SerializationFormResolver<ApplicationUserAccessToken<'_>> {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    pub fn serialize<'a>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        application_user_access_token: &'a ApplicationUserAccessToken<'_>,
    ) -> Result<ApplicationUserAccessTokenEncrypted, ErrorAuditor> {
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

        let application_user_access_token_signature = Encoder::<Signature>::encode(
            pushable_environment_configuration,
            application_user_access_token_serialized.as_str(),
        );

        let application_user_access_token_encrypted = format!(
            "{}{}{}",
            application_user_access_token_serialized.as_str(),
            Self::TOKEN_PARTS_SEPARATOR,
            application_user_access_token_signature.as_str()
        );

        return Ok(
            ApplicationUserAccessTokenEncrypted::new(application_user_access_token_encrypted)
        );
    }

    pub fn deserialize<'a>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        application_user_access_token_encrypted: &'a ApplicationUserAccessTokenEncrypted,
    ) -> Result<InvalidArgumentResult<ApplicationUserAccessToken<'static>>, ErrorAuditor> {
        let mut token_part_registry = application_user_access_token_encrypted
            .get()
            .splitn::<'_, &'_ str>(2, Self::TOKEN_PARTS_SEPARATOR);

        let invalid_argument_result = InvalidArgumentResult::InvalidArgument {
            invalid_argument: InvalidArgument::ApplicationUserAccessTokenEncrypted,
        };

        let application_user_access_token_serialized = match token_part_registry.next() {
            Some(application_user_access_token_serialized_) => application_user_access_token_serialized_,
            None => {
                return Ok(invalid_argument_result);
            }
        };

        let application_user_access_token_signature = match token_part_registry.next() {
            Some(application_user_access_token_signature_) => application_user_access_token_signature_,
            None => {
                return Ok(invalid_argument_result);
            }
        };

        let is_valid = Encoder::<Signature>::is_valid(
            pushable_environment_configuration,
            application_user_access_token_serialized,
            application_user_access_token_signature,
        );

        if !is_valid {
            return Ok(invalid_argument_result);
        }

        let data = match Encoder_::<Base64>::decode(application_user_access_token_serialized.as_bytes()) {
            Ok(data_) => data_,
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
    fn encode<'a>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        application_user_access_token_serialized: &'a str,
    ) -> String {
        let mut hmac_encoded_data: Vec<u8> = vec![];

        Encoder_::<Hmac>::encode(
            pushable_environment_configuration
                .encryption
                .private_key
                .application_user_access_token
                .as_bytes(),
            application_user_access_token_serialized.as_bytes(),
            hmac_encoded_data.as_mut_slice(),
        );

        return Encoder_::<Base64>::encode(hmac_encoded_data.as_slice());
    }

    fn is_valid<'a>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        application_user_access_token_serialized: &'a str,
        application_user_access_token_signature: &'a str,
    ) -> bool {
        let application_user_access_token_signature_ = Self::encode(
            pushable_environment_configuration,
            application_user_access_token_serialized,
        );

        return application_user_access_token_signature_.as_str() == application_user_access_token_signature;
    }
}
