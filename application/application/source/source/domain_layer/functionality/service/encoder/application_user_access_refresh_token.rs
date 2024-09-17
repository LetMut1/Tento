use super::Encoder;
use crate::{
    domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken,
    infrastructure_layer::{
        data::{
            control_type::MessagePack,
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
use application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use aggregate_error::AggregateError;
impl Encoder<ApplicationUserAccessRefreshToken<'_>> {
    pub fn to_encrypted<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
    ) -> Result<ApplicationUserAccessRefreshTokenEncrypted, AggregateError> {
        return Ok(
            ApplicationUserAccessRefreshTokenEncrypted(
                Encoder_::<HmacSha3_512>::encode(
                    environment_configuration.encryption.private_key.application_user_access_refresh_token.as_bytes(),
                    Serializer::<MessagePack>::serialize(application_user_access_refresh_token)?.as_slice(),    // TODO TODO TODO Serializer::<MessagePack> - Нужен любой фаст алгоритм сериализации.
                )?,
            ),
        );
    }
    pub fn is_valid<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
        application_user_access_refresh_token_encrypted: &'a ApplicationUserAccessRefreshTokenEncrypted,
    ) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha3_512>::is_valid(
            environment_configuration.encryption.private_key.application_user_access_refresh_token.as_bytes(),
            Serializer::<MessagePack>::serialize(application_user_access_refresh_token)?.as_slice(),
            application_user_access_refresh_token_encrypted.0.as_slice(),
        );
    }
}
