use super::Encoder;
use crate::{
    domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken,
    infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
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
use dedicated_crate::user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded;
impl Encoder<UserAccessRefreshToken<'_>> {
    pub fn encode<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        user_access_refresh_token: &'a UserAccessRefreshToken<'_>,
    ) -> Result<UserAccessRefreshTokenEncoded, AggregateError> {
        return Result::Ok(
            UserAccessRefreshTokenEncoded(
                Encoder_::<HmacSha3_512>::encode(
                    environment_configuration.encryption.private_key.user_access_refresh_token.as_bytes(),
                    Serializer::<BitCode>::serialize(
                        &Token {
                            user__id: user_access_refresh_token.user__id,
                            user_device__id: user_access_refresh_token.user_device__id,
                            user_access_token__id: user_access_refresh_token.user_access_token__id.as_ref(),
                            user_access_token__obfuscation_value: user_access_refresh_token.obfuscation_value.as_str(),
                            user_access_token__expires_at: user_access_refresh_token.expires_at,
                            user_access_token__updated_at: user_access_refresh_token.updated_at,
                        },
                    )?
                    .as_slice(),
                )?,
            ),
        );
    }
    pub fn is_valid<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        user_access_refresh_token: &'a UserAccessRefreshToken<'_>,
        user_access_refresh_token_encoded: &'a UserAccessRefreshTokenEncoded,
    ) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha3_512>::is_valid(
            environment_configuration.encryption.private_key.user_access_refresh_token.as_bytes(),
            Serializer::<BitCode>::serialize(
                &Token {
                    user__id: user_access_refresh_token.user__id,
                    user_device__id: user_access_refresh_token.user_device__id,
                    user_access_token__id: user_access_refresh_token.user_access_token__id.as_ref(),
                    user_access_token__obfuscation_value: user_access_refresh_token.obfuscation_value.as_str(),
                    user_access_token__expires_at: user_access_refresh_token.expires_at,
                    user_access_token__updated_at: user_access_refresh_token.updated_at,
                },
            )?
            .as_slice(),
            user_access_refresh_token_encoded.0.as_slice(),
        );
    }
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
struct Token<'a> {
    user__id: i64,
    user_device__id: &'a str,
    user_access_token__id: &'a str,
    user_access_token__obfuscation_value: &'a str,
    user_access_token__expires_at: i64,
    user_access_token__updated_at: i64,
}
