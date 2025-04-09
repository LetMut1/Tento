use {
    super::Encoder,
    crate::{
        domain_layer::data::entity::channel_subscription_token::ChannelSubscriptionToken,
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
    dedicated::channel_subscription_token_signed::ChannelSubscriptionTokenSigned,
};
impl Encoder<ChannelSubscriptionToken> {
    pub fn encode(
        private_key: &'static PrivateKey,
        user__id: i64,
        channel__id: i64,
        channel_subscription_token__obfuscation_value: i64,
        channel_subscription_token__expires_at: i64,
    ) -> Result<ChannelSubscriptionTokenSigned, AggregateError> {
        let serialized = Serializer::<BitCode>::serialize(
            &Data {
                user__id,
                channel__id,
                channel_subscription_token__obfuscation_value,
                channel_subscription_token__expires_at,
            },
        )?;
        let signature = Encoder_::<HmacSha2_256>::encode(
            private_key.channel_subscription_token.as_bytes(),
            serialized.as_slice(),
        )?;
        return Result::Ok(
            ChannelSubscriptionTokenSigned {
                channel__id,
                channel_subscription_token__obfuscation_value,
                channel_subscription_token__expires_at,
                signature,
            },
        );
    }
    pub fn is_valid<'a>(
        private_key: &'static PrivateKey,
        user__id: i64,
        channel_subscription_token_signed: &'a ChannelSubscriptionTokenSigned,
    ) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha2_256>::is_valid(
            private_key.channel_subscription_token.as_bytes(),
            Serializer::<BitCode>::serialize(
                &Data {
                    user__id,
                    channel__id: channel_subscription_token_signed.channel__id,
                    channel_subscription_token__obfuscation_value: channel_subscription_token_signed.channel_subscription_token__obfuscation_value,
                    channel_subscription_token__expires_at: channel_subscription_token_signed.channel_subscription_token__expires_at,
                },
            )?
            .as_slice(),
            channel_subscription_token_signed.signature.as_slice(),
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
    channel_subscription_token__obfuscation_value: i64,
    channel_subscription_token__expires_at: i64,
}
