use {
    super::Encoder,
    crate::{
        domain_layer::data::entity::channel_publication1_token::ChannelPublication1Token,
        infrastructure_layer::{
            data::{
                aggregate_error::AggregateError,
                environment_configuration::run_server::PrivateKey,
            },
            functionality::service::{
                encoder::{
                    Encoder as Encoder_,
                    HmacSha2_256,
                },
                serializer::{
                    BitCode,
                    Serialize,
                    Serializer,
                },
            },
        },
    },
    dedicated::channel_publication1_token_signed::ChannelPublication1TokenSigned,
};
impl Encoder<ChannelPublication1Token> {
    pub fn encode(
        private_key: &'static PrivateKey,
        user__id: i64,
        channel_publication1__id: i64,
        channel_publication1__obfuscation_value: i64,
        channel_publication1_token__expires_at: i64,
    ) -> Result<ChannelPublication1TokenSigned, AggregateError> {
        let serialized = Serializer::<BitCode>::serialize(
            &Data {
                user__id,
                channel_publication1__id,
                channel_publication1__obfuscation_value,
                channel_publication1_token__expires_at,
            },
        )?;
        let signature = Encoder_::<HmacSha2_256>::encode(
            private_key.channel_publication1_token.as_bytes(),
            serialized.as_slice(),
        )?;
        return Result::Ok(
            ChannelPublication1TokenSigned {
                channel_publication1__id,
                channel_publication1__obfuscation_value,
                channel_publication1_token__expires_at,
                signature,
            },
        );
    }
    pub fn is_valid<'a>(private_key: &'static PrivateKey, user__id: i64, channel_publication1_token_signed: &'a ChannelPublication1TokenSigned) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha2_256>::is_valid(
            private_key.channel_publication1_token.as_bytes(),
            Serializer::<BitCode>::serialize(
                &Data {
                    user__id,
                    channel_publication1__id: channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1__obfuscation_value: channel_publication1_token_signed.channel_publication1__obfuscation_value,
                    channel_publication1_token__expires_at: channel_publication1_token_signed.channel_publication1_token__expires_at,
                },
            )?
            .as_slice(),
            channel_publication1_token_signed.signature.as_slice(),
        );
    }
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(serde::Serialize)
)]
#[derive(bitcode::Encode)]
struct Data {
    user__id: i64,
    channel_publication1__id: i64,
    channel_publication1__obfuscation_value: i64,
    channel_publication1_token__expires_at: i64,
}
