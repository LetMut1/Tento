use super::FormResolver;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::encoder::base64::Base64;
use crate::infrastructure_layer::functionality::service::encoder::Encoder as Encoder_;
use crate::infrastructure_layer::functionality::service::encoder::hmac::HmacSha3512;
use crate::infrastructure_layer::functionality::service::serializer::message_pack::MessagePack;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;

impl FormResolver<ApplicationUserAccessToken<'_>> {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    pub fn to_encrypted<'a>(environment_configuration: &'static EnvironmentConfiguration, application_user_access_token: &'a ApplicationUserAccessToken<'_>) -> Result<ApplicationUserAccessTokenEncrypted, Auditor<Error>> {
        let data = Serializer::<MessagePack>::serialize(application_user_access_token)?;

        let application_user_access_token_serialized = Encoder_::<Base64>::encode(data.as_slice());

        let application_user_access_token_serialized_signature = Encoder::<Signature>::encode(
            environment_configuration,
            application_user_access_token_serialized.as_bytes()
        )?;

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

    pub fn from_encrypted<'a>(environment_configuration: &'static EnvironmentConfiguration, application_user_access_token_encrypted: &'a ApplicationUserAccessTokenEncrypted) -> Result<InvalidArgumentResult<ApplicationUserAccessToken<'static>>, Auditor<Error>> {
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

        if !Encoder::<Signature>::is_valid(
            environment_configuration,
            application_user_access_token_serialized.as_bytes(),
            application_user_access_token_serialized_signature.as_bytes(),
        )?
        {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserAccessTokenEncrypted,
                }
            );
        }

        let data = Encoder_::<Base64>::decode(application_user_access_token_serialized.as_bytes())?;

        let application_user_access_token = Serializer::<MessagePack>::deserialize::<'_, ApplicationUserAccessToken<'static>>(data.as_slice())?;

        return Ok(
            InvalidArgumentResult::Ok {
                subject: application_user_access_token,
            },
        );
    }
}

struct Signature;

impl Encoder<Signature> {
    fn encode<'a>(environment_configuration: &'static EnvironmentConfiguration, application_user_access_token_serialized: &'a [u8]) -> Result<String, Auditor<Error>> {
        let application_user_access_token_serialized_encoded = Encoder_::<HmacSha3512>::encode(
            environment_configuration.encryption.private_key.application_user_access_token.as_bytes(),
            application_user_access_token_serialized,
        )?;

        let application_user_access_token_serialized_signature = Encoder_::<Base64>::encode(application_user_access_token_serialized_encoded.into_bytes().as_slice());

        return Ok(application_user_access_token_serialized_signature);
    }

    fn is_valid<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_token_serialized: &'a [u8],
        application_user_access_token_serialized_signature: &'a [u8],
    ) -> Result<bool, Auditor<Error>> {
        let application_user_access_token_serialized_encoded = Encoder_::<Base64>::decode(application_user_access_token_serialized_signature)?;

        return Ok(
            Encoder_::<HmacSha3512>::is_valid(
                environment_configuration.encryption.private_key.application_user_access_token.as_bytes(),
                application_user_access_token_serialized,
                application_user_access_token_serialized_encoded.as_slice(),
            )?
        );
    }
}
