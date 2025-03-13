// use {
//     super::Encoder,
//     crate::{
//         domain_layer::data::entity::channel_publication1_token::ChannelPublication1Token,
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
//     dedicated::channel_publication1_token_signed::ChannelPublication1TokenSigned,
// };
// impl Encoder<ChannelPublication1Token> {
//     pub fn encode(
//         user__id: i64,
//         channel_publication1__id: i64,
//         channel_publication1_token__expires_at: i64,
//     ) -> Result<ChannelPublication1TokenSigned, AggregateError> {
//         return Result::Ok(
//             ChannelTokenHashed {
//                 channel_token__expires_at,
//                 hash: Encoder_::<Highway>::encode(
//                     [
//                         channel__obfuscation_value.abs() as u64,
//                         channel_token__expires_at.abs() as u64,
//                         user__id.abs() as u64,
//                         channel__id.abs() as u64,
//                     ],
//                     Serializer::<BitCode>::serialize(
//                         &Data {
//                             user__id,
//                             channel__id,
//                             channel__obfuscation_value,
//                             channel_token__expires_at,
//                         }
//                     )?.as_slice(),
//                 ),
//             },
//         );
//     }
//     pub fn is_valid<'a>(
//         user__id: i64,
//         channel__id: i64,
//         channel__obfuscation_value: i64,
//         channel_token_hashed: &'a ChannelTokenHashed,
//     ) -> Result<bool, AggregateError> {
//         return Result::Ok(
//             Self::encode(
//                 user__id,
//                 channel__id,
//                 channel__obfuscation_value,
//                 channel_token_hashed.channel_token__expires_at,
//             )?.hash == channel_token_hashed.hash,
//         );
//     }
// }
// #[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize))]
// #[derive(bitcode::Encode)]
// struct Data {
//     user__id: i64,
//     channel__id: i64,
//     channel__obfuscation_value: i64,
//     channel_token__expires_at: i64,
// }
