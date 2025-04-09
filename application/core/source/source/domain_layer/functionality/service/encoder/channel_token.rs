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
const CT: &'static str = "sdsdc";
impl Encoder<ChannelToken> {
    pub fn encode(
        // private_key: &'static PrivateKey,
        user__id: i64,
        channel__id: i64,
        channel_token__obfuscation_value: i64,
        channel_token__expires_at: i64,
        channel_token__is_channel_subscription_exist: bool,
    ) -> Result<ChannelTokenSigned, AggregateError> {
        let serialized = Serializer::<BitCode>::serialize(
            &Data {
                user__id,
                channel__id,
                channel_token__obfuscation_value,
                channel_token__expires_at,
                channel_token__is_channel_subscription_exist,
            },
        )?;
        let signature = Encoder_::<HmacSha2_256>::encode(
            // private_key.channel_publication1_token.as_bytes(),
            CT.as_bytes(),
            serialized.as_slice(),
        )?;
        return Result::Ok(
            ChannelTokenSigned {
                channel__id,
                channel_token__obfuscation_value,
                channel_token__expires_at,
                channel_token__is_channel_subscription_exist,
                signature,
            },
        );
    }
    pub fn is_valid<'a>(
        // private_key: &'static PrivateKey,
        user__id: i64,
        channel_token_signed: &'a ChannelTokenSigned,
    ) -> Result<bool, AggregateError> {
        return Encoder_::<HmacSha2_256>::is_valid(
            // private_key.channel_publication1_token.as_bytes(),
            CT.as_bytes(),
            Serializer::<BitCode>::serialize(
                &Data {
                    user__id,
                    channel__id: channel_token_signed.channel__id,
                    channel_token__obfuscation_value: channel_token_signed.channel_token__obfuscation_value,
                    channel_token__expires_at: channel_token_signed.channel_token__expires_at,
                    channel_token__is_channel_subscription_exist: channel_token_signed.channel_token__is_channel_subscription_exist,
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
    channel_token__is_channel_subscription_exist: bool,
}
