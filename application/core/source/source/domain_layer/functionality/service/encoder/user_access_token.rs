use {
    super::Encoder,
    crate::{
        domain_layer::data::entity::user_access_token::UserAccessToken,
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
    dedicated::user_access_token_signed::{
        UserAccessTokenSigned,
        UserAccessTokenSigned_,
    },
};
impl Encoder<UserAccessToken> {
    pub fn encode<'a>(
        private_key: &'static PrivateKey,
        user_access_token__id: &'a str,
        user__id: i64,
        user_device__id: &'a str,
        user_access_token__expires_at: i64,
    ) -> Result<UserAccessTokenSigned_, AggregateError> {
        let serialized = Serializer::<BitCode>::serialize(
            &Data {
                user_access_token__id,
                user__id,
                user_device__id,
                user_access_token__expires_at,
            },
        )?;
        let signature = Encoder_::<HmacSha3_512>::encode(
            private_key.user_access_token.as_bytes(),
            serialized.as_slice(),
        )?;
        return Result::Ok(
            UserAccessTokenSigned_ {
                user_access_token__id: user_access_token__id.to_string(),
                user__id,
                user_device__id: user_device__id.to_string(),
                user_access_token__expires_at,
                signature,
            },
        );
    }
    // user_access_token__id: &'a str,
    // user__id: i64,
    // user_device__id: &'a str,
    // user_access_token__expires_at: i64,
    pub fn decode<'a>(private_key: &'static PrivateKey, user_access_token_signed: &'a UserAccessTokenSigned) -> Result<(&'a str, i64, &'a str, i64), AggregateError> {
        if !Encoder_::<HmacSha3_512>::is_valid(
            private_key.user_access_token.as_bytes(),
            Serializer::<BitCode>::serialize(
                &Data {
                    user_access_token__id: user_access_token_signed.user_access_token__id,
                    user__id: user_access_token_signed.user__id,
                    user_device__id: user_access_token_signed.user_device__id,
                    user_access_token__expires_at: user_access_token_signed.user_access_token__expires_at,
                },
            )?
            .as_slice(),
            user_access_token_signed.singature.as_slice(),
        )? {
            return Result::Err(crate::new_invalid_argument!());
        }
        return Result::Ok(
            (
                user_access_token_signed.user_access_token__id,
                user_access_token_signed.user__id,
                user_access_token_signed.user_device__id,
                user_access_token_signed.user_access_token__expires_at,
            ),
        )
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
    user_access_token__id: &'a str,
    user__id: i64,
    user_device__id: &'a str,
    user_access_token__expires_at: i64,
}