use super::Encoder;
use crate::{
    domain_layer::data::entity::user_access_token::UserAccessToken,
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
            },
            environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::{
            encoder::{
                Encoder as Encoder_,
                HmacSha3_512,
            },
            serializer::{
                BitCode,
                Serialize,
                Serializer,
            },
        },
    },
};
use dedicated_crate::user_access_token_encoded::UserAccessTokenEncoded;
impl Encoder<UserAccessToken<'_>> {
    pub fn encode<'a>(environment_configuration: &'static EnvironmentConfiguration, user_access_token: &'a UserAccessToken<'_>) -> Result<UserAccessTokenEncoded, AggregateError> {
        let user_access_token_serialized = Serializer::<BitCode>::serialize(user_access_token)?;
        let user_access_token_encoded = Encoder_::<HmacSha3_512>::encode(
            environment_configuration.encryption.private_key.user_access_token.as_bytes(),
            user_access_token_serialized.as_slice(),
        )?;
        return Result::Ok(
            UserAccessTokenEncoded {
                serialized: user_access_token_serialized,
                encoded: user_access_token_encoded,
            },
        );
    }
    pub fn decode<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        user_access_token_encoded: &'a UserAccessTokenEncoded,
    ) -> Result<UserAccessToken<'a>, AggregateError> {
        if !Encoder_::<HmacSha3_512>::is_valid(
            environment_configuration.encryption.private_key.user_access_token.as_bytes(),
            user_access_token_encoded.serialized.as_slice(),
            user_access_token_encoded.encoded.as_slice(),
        )? {
            return Result::Err(
                AggregateError::new_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        return Serializer::<BitCode>::deserialize::<'_, UserAccessToken<'a>>(user_access_token_encoded.serialized.as_slice());
    }
}
