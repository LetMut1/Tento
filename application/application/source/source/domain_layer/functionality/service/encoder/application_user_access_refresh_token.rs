use super::Encoder;
use crate::{
    domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken,
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
use aggregate_error::AggregateError;
use application_user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded;
impl Encoder<UserAccessRefreshToken<'_>> {
    pub fn encode<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_refresh_token: &'a UserAccessRefreshToken<'_>,
    ) -> Result<UserAccessRefreshTokenEncoded, AggregateError> {
        return Result::Ok(
            UserAccessRefreshTokenEncoded(
                Encoder_::<HmacSha3_512>::encode(
                    environment_configuration.encryption.private_key.application_user_access_refresh_token.as_bytes(),
                    Serializer::<MessagePack>::serialize(application_user_access_refresh_token)?.as_slice(), // TODO TODO TODO Serializer::<MessagePack> - Нужен любой фаст алгоритм сериализации.
                )?,
            ),
        );
    }
    pub fn is_valid<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_refresh_token: &'a UserAccessRefreshToken<'_>,
        application_user_access_refresh_token_encoded: &'a UserAccessRefreshTokenEncoded,
    ) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha3_512>::is_valid(
            environment_configuration.encryption.private_key.application_user_access_refresh_token.as_bytes(),
            Serializer::<MessagePack>::serialize(application_user_access_refresh_token)?.as_slice(),
            application_user_access_refresh_token_encoded.0.as_slice(),
        );
    }
}
