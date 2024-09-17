use super::Encoder;
use crate::{
    domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken,
    infrastructure_layer::{
        data::environment_configuration::environment_configuration::EnvironmentConfiguration,
        functionality::service::{
            encoder::{
                hmac::HmacSha3_512,
                Encoder as Encoder_,
            },
            serializer::{
                message_pack::MessagePack,
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
use application_user_access_token_encoded::ApplicationUserAccessTokenEncoded;
impl Encoder<ApplicationUserAccessToken<'_>> {
    pub fn encode<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_token: &'a ApplicationUserAccessToken<'_>,
    ) -> Result<ApplicationUserAccessTokenEncoded, AggregateError> {
        let application_user_access_token_serialized = Serializer::<MessagePack>::serialize(application_user_access_token)?; // TODO TODO TODO Serializer::<MessagePack> - Нужен любой фаст алгоритм сериализации.
        let application_user_access_token_encoded = Encoder_::<HmacSha3_512>::encode(
            environment_configuration.encryption.private_key.application_user_access_token.as_bytes(),
            application_user_access_token_serialized.as_slice(),
        )?;
        return Ok(
            ApplicationUserAccessTokenEncoded {
                serialized: application_user_access_token_serialized,
                encoded: application_user_access_token_encoded,
            },
        );
    }
    pub fn decode<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_token_encoded: &'a ApplicationUserAccessTokenEncoded,
    ) -> Result<ApplicationUserAccessToken<'static>, AggregateError> {
        if !Encoder_::<HmacSha3_512>::is_valid(
            environment_configuration.encryption.private_key.application_user_access_token.as_bytes(),
            application_user_access_token_encoded.serialized.as_slice(),
            application_user_access_token_encoded.encoded.as_slice(),
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
        return Serializer::<MessagePack>::deserialize::<'_, ApplicationUserAccessToken<'static>>(application_user_access_token_encoded.serialized.as_slice());
    }
}
