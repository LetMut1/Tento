use super::Encoder;
use crate::{
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
};
use dedicated::user_access_token_encoded::UserAccessTokenEncoded;
impl Encoder<UserAccessToken<'_>> {
    pub fn encode<'a>(
        private_key: &'static PrivateKey,
        user_access_token: &'a UserAccessToken<'_>
    ) -> Result<UserAccessTokenEncoded, AggregateError> {
        let user_access_token_serialized = Serializer::<BitCode>::serialize(user_access_token)?;
        let user_access_token_encoded = Encoder_::<HmacSha3_512>::encode(
            private_key.user_access_token.as_bytes(),
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
        private_key: &'static PrivateKey,
        user_access_token_encoded: &'a UserAccessTokenEncoded,
    ) -> Result<UserAccessToken<'a>, AggregateError> {
        if !Encoder_::<HmacSha3_512>::is_valid(
            private_key.user_access_token.as_bytes(),
            user_access_token_encoded.serialized.as_slice(),
            user_access_token_encoded.encoded.as_slice(),
        )? {
            return Result::Err(crate::new_invalid_argument!());
        }
        return Serializer::<BitCode>::deserialize::<'_, UserAccessToken<'a>>(user_access_token_encoded.serialized.as_slice());
    }
}
