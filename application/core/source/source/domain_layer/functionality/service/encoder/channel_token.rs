use {
    super::Encoder,
    crate::{
        domain_layer::data::entity::channel_token::ChannelToken,
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
    dedicated::channel_token_signed::ChannelTokenSigned,
};
impl Encoder<ChannelToken> {
    pub fn encode(
        private_key: &'static PrivateKey,
        user__id: i64,
        channel__id: i64,
        channel_token__obfuscation_value: i64,
        channel_token__expires_at: i64,
        channel_token__is_user_the_channel_subscriber: bool,
        channel_token__is_user_the_channel_owner: bool,
    ) -> Result<ChannelTokenSigned, AggregateError> {
        let serialized = Serializer::<BitCode>::serialize(
            &Data {
                user__id,
                channel__id,
                channel_token__obfuscation_value,
                channel_token__expires_at,
                channel_token__is_user_the_channel_subscriber,
                channel_token__is_user_the_channel_owner,
            },
        )?;
        let signature = Encoder_::<HmacSha2_256>::encode(
            private_key.channel_token.as_bytes(),
            serialized.as_slice(),
        )?;
        return Result::Ok(
            ChannelTokenSigned {
                channel__id,
                channel_token__obfuscation_value,
                channel_token__expires_at,
                channel_token__is_user_the_channel_subscriber,
                channel_token__is_user_the_channel_owner,
                signature,
            },
        );
    }
    pub fn is_valid<'a>(
        private_key: &'static PrivateKey,
        user__id: i64,
        channel_token_signed: &'a ChannelTokenSigned,
    ) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha2_256>::is_valid(
            private_key.channel_token.as_bytes(),
            Serializer::<BitCode>::serialize(
                &Data {
                    user__id,
                    channel__id: channel_token_signed.channel__id,
                    channel_token__obfuscation_value: channel_token_signed.channel_token__obfuscation_value,
                    channel_token__expires_at: channel_token_signed.channel_token__expires_at,
                    channel_token__is_user_the_channel_subscriber: channel_token_signed.channel_token__is_user_the_channel_subscriber,
                    channel_token__is_user_the_channel_owner: channel_token_signed.channel_token__is_user_the_channel_owner,
                },
            )?
            .as_slice(),
            channel_token_signed.signature.as_slice(),
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
    channel__id: i64,
    channel_token__obfuscation_value: i64,
    channel_token__expires_at: i64,
    channel_token__is_user_the_channel_subscriber: bool,
    channel_token__is_user_the_channel_owner: bool,
}
