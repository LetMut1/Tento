use super::FormResolver;
use crate::{
    domain_layer::{
        data::entity::application_user_access_token::ApplicationUserAccessToken,
        functionality::service::encoder::Encoder,
    },
    infrastructure_layer::{
        data::{
            control_type::{
                Base64,
                MessagePack,
            },
            environment_configuration::environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::{
            encoder::{
                hmac::HmacSha3_512,
                Encoder as Encoder_,
            },
            serializer::{
                Serialize,
                Serializer,
            },
        },
    },
};
use aggregate_error::{
    AggregateError,
    Backtrace,
};
impl FormResolver<ApplicationUserAccessToken<'_>> {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";
    pub fn to_encrypted<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_token: &'a ApplicationUserAccessToken<'_>,
    ) -> Result<String, AggregateError> {
        let data = Serializer::<MessagePack>::serialize(application_user_access_token)?;
        let application_user_access_token_serialized = Encoder_::<Base64>::encode(data.as_slice());
        let application_user_access_token_serialized_signature = Encoder::<Signature>::encode(
            environment_configuration,
            application_user_access_token_serialized.as_bytes(),
        )?;
        return Ok(
            format!(
                "{}{}{}",
                application_user_access_token_serialized.as_str(),
                Self::TOKEN_PARTS_SEPARATOR,
                application_user_access_token_serialized_signature.as_str()
            ),
        );
    }
    pub fn from_encrypted<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_token_encrypted: &'a str,
    ) -> Result<ApplicationUserAccessToken<'static>, AggregateError> {
        let mut token_part_registry = application_user_access_token_encrypted.splitn::<'_, &'_ str>(
            2,
            Self::TOKEN_PARTS_SEPARATOR,
        );
        let application_user_access_token_serialized = token_part_registry.next().ok_or_else(
            || -> _ {
                return AggregateError::new_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                );
            },
        )?;
        let application_user_access_token_serialized_signature = token_part_registry.next().ok_or_else(
            || -> _ {
                return AggregateError::new_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                );
            },
        )?;
        if !Encoder::<Signature>::is_valid(
            environment_configuration,
            application_user_access_token_serialized.as_bytes(),
            application_user_access_token_serialized_signature.as_bytes(),
        )? {
            return Err(
                AggregateError::new_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        let data = Encoder_::<Base64>::decode(application_user_access_token_serialized.as_bytes())?;
        return Serializer::<MessagePack>::deserialize::<'_, ApplicationUserAccessToken<'static>>(data.as_slice());
    }
}
struct Signature;
impl Encoder<Signature> {
    fn encode<'a>(environment_configuration: &'static EnvironmentConfiguration, application_user_access_token_serialized: &'a [u8]) -> Result<String, AggregateError> {
        let application_user_access_token_serialized_encoded = Encoder_::<HmacSha3_512>::encode(
            environment_configuration.encryption.private_key.application_user_access_token.as_bytes(),
            application_user_access_token_serialized,
        )?;
        return Ok(Encoder_::<Base64>::encode(application_user_access_token_serialized_encoded.into_bytes().as_slice()));
    }
    fn is_valid<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_token_serialized: &'a [u8],
        application_user_access_token_serialized_signature: &'a [u8],
    ) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha3_512>::is_valid(
            environment_configuration.encryption.private_key.application_user_access_token.as_bytes(),
            application_user_access_token_serialized,
            Encoder_::<Base64>::decode(application_user_access_token_serialized_signature)?.as_slice(),
        );
    }
}
