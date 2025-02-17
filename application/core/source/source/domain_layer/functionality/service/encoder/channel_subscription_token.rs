// use {
//     super::Encoder,
//     crate::{
//         domain_layer::data::entity::channel_subscription_token::ChannelSubscriptionToken,
//         infrastructure_layer::{
//             data::aggregate_error::AggregateError,
//             functionality::service::{
//                 encoder::{
//                     Encoder as Encoder_,
//                     Highway,
//                 },
//                 serializer::{
//                     BitCode,
//                     Serialize,
//                     Serializer,
//                 },
//             },
//         },
//     },
//     dedicated::channel_subscription_token_encoded::ChannelSubscriptionTokenEncoded,
// };
// impl Encoder<ChannelSubscriptionToken> {
//     pub fn encode<'a>(channel_subscription_token: &'a ChannelSubscriptionToken) -> Result<ChannelSubscriptionTokenEncoded, AggregateError> {
//         return Result::Ok(
//             ChannelSubscriptionTokenEncoded(
//                 Encoder_::<Highway>::encode(
//                     [
//                         channel_subscription_token.user__id.abs() as u64,
//                         channel_subscription_token.channel__id.abs() as u64,
//                         channel_subscription_token.channel__obfuscation_value.abs() as u64,



//                         todo!(),
//                         channel_subscription_token.created_at.abs() as u64,
//                     ],
//                     Serializer::<BitCode>::serialize(channel_subscription_token)?.as_slice(),
//                 ),
//             ),
//         );
//     }
//     pub fn is_valid<'a>(
//         channel_subscription_token: &'a ChannelSubscriptionToken,
//         channel_subscription_token_encoded: ChannelSubscriptionTokenEncoded,
//     ) -> Result<bool, AggregateError> {
//         return Result::Ok(
//             Self::encode(channel_subscription_token)?.0 == channel_subscription_token_encoded.0
//         );
//     }
// }
