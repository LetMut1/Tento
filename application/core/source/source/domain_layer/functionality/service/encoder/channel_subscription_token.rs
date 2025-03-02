use {
    super::Encoder,
    crate::{
        domain_layer::data::entity::channel_subscription_token::ChannelSubscriptionToken,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::{
                encoder::{
                    Encoder as Encoder_,
                    Highway,
                },
                serializer::{
                    BitCode,
                    Serialize,
                    Serializer,
                },
            },
        },
    },
    dedicated::channel_subscription_token_hashed::ChannelSubscriptionTokenHashed,
};
impl Encoder<ChannelSubscriptionToken> {
    pub fn encode(
        user__id: i64,
        channel__id: i64,
        channel__obfuscation_value: i64,
        channel_subscription_token__expires_at: i64,
    ) -> Result<ChannelSubscriptionTokenHashed, AggregateError> {
        return Result::Ok(
            ChannelSubscriptionTokenHashed {
                channel_subscription_token__expires_at,
                hash: Encoder_::<Highway>::encode(
                    [
                        user__id.abs() as u64,
                        channel__id.abs() as u64,
                        channel__obfuscation_value.abs() as u64,
                        channel_subscription_token__expires_at.abs() as u64,
                    ],
                    Serializer::<BitCode>::serialize(
                        &Data {
                            user__id,
                            channel__id,
                            channel__obfuscation_value,
                            channel_subscription_token__expires_at,
                        }
                    )?.as_slice(),
                ),
            },
        );
    }
    pub fn is_valid<'a>(
        user__id: i64,
        channel__id: i64,
        channel__obfuscation_value: i64,
        channel_subscription_token_hashed: &'a ChannelSubscriptionTokenHashed,
    ) -> Result<bool, AggregateError> {
        return Result::Ok(
            Self::encode(
                user__id,
                channel__id,
                channel__obfuscation_value,
                channel_subscription_token_hashed.channel_subscription_token__expires_at,
            )?.hash == channel_subscription_token_hashed.hash,
        );
    }
}
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize))]
#[derive(bitcode::Encode)]
struct Data {
    user__id: i64,
    channel__id: i64,
    channel__obfuscation_value: i64,
    channel_subscription_token__expires_at: i64,
}