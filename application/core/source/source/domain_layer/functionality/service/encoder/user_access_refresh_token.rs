use {
    super::Encoder,
    crate::{
        domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken,
        infrastructure_layer::{
            data::{
                aggregate_error::AggregateError,
                environment_configuration::run_server::PrivateKey,
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
    },
    dedicated::user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded,
};
impl Encoder<UserAccessRefreshToken> {
    pub fn encode<'a>(
        private_key: &'static PrivateKey,
        user__id: i64,
        user_device__id: &'a str,
        user_access_token__id: &'a str,
        user_access_refresh_token__obfuscation_value: &'a str,
        user_access_refresh_token__expires_at: i64,
        user_access_refresh_token__updated_at: i64,
    ) -> Result<UserAccessRefreshTokenEncoded, AggregateError> {
        return Result::Ok(
            UserAccessRefreshTokenEncoded(
                Encoder_::<HmacSha3_512>::encode(
                    private_key.user_access_refresh_token.as_bytes(),
                    Serializer::<BitCode>::serialize(
                        &Data {
                            user__id,
                            user_device__id,
                            user_access_token__id,
                            user_access_refresh_token__obfuscation_value,
                            user_access_refresh_token__expires_at,
                            user_access_refresh_token__updated_at,
                        },
                    )?
                    .as_slice(),
                )?,
            ),
        );
    }
    pub fn is_valid<'a>(
        private_key: &'static PrivateKey,
        user__id: i64,
        user_device__id: &'a str,
        user_access_token__id: &'a str,
        user_access_refresh_token__obfuscation_value: &'a str,
        user_access_refresh_token__expires_at: i64,
        user_access_refresh_token__updated_at: i64,
        user_access_refresh_token_encoded: &'a UserAccessRefreshTokenEncoded,
    ) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha3_512>::is_valid(
            private_key.user_access_refresh_token.as_bytes(),
            Serializer::<BitCode>::serialize(
                &Data {
                    user__id,
                    user_device__id,
                    user_access_token__id,
                    user_access_refresh_token__obfuscation_value,
                    user_access_refresh_token__expires_at,
                    user_access_refresh_token__updated_at,
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
struct Data<'a> {
    user__id: i64,
    user_device__id: &'a str,
    user_access_token__id: &'a str,
    user_access_refresh_token__obfuscation_value: &'a str,
    user_access_refresh_token__expires_at: i64,
    user_access_refresh_token__updated_at: i64,
}