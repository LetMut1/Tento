// use {
//     crate::{
//         application_layer::functionality::action_processor::{
//             ActionProcessor,
//             ActionProcessor_,
//             Inner,
//         },
//         domain_layer::{
//             data::entity::{
//                 channel::{
//                     Channel,
//                     Channel_Id,
//                 },
//                 user_access_token::UserAccessToken,
//                 channel_publication1::{
//                     ChannelPublication1,
//                     ChannelPublication1_ImagesPathes,
//                     ChannelPublication1_Text,
//                     ChannelPublication1_Id,
//                 },
//                 channel_publication1_mark::ChannelPublication1Mark,
//             },
//             functionality::service::{
//                 extractor::{
//                     Extracted,
//                     Extractor,
//                 },
//                 validator::Validator,
//             },
//         },
//         infrastructure_layer::{
//             data::aggregate_error::AggregateError,
//             functionality::{
//                 repository::{
//                     postgresql::{
//                         ChannelBy1,
//                         ChannelPublication1Insert,
//                         Postgresql,
//                     },
//                     Repository,
//                 },
//                 service::resolver::{
//                     Resolver,
//                     UnixTime,
//                 }
//             },
//         },
//     },
//     dedicated::{
//         action_processor_incoming_outcoming::action_processor::channel_publication1_mark::create::{
//             Incoming,
//             Precedent,
//         },
//         unified_report::UnifiedReport,
//         void::Void,
//     },
//     std::future::Future,
// };
// pub struct ChannelPublication1Mark_Create;
// impl ActionProcessor_ for ActionProcessor<ChannelPublication1Mark_Create> {
//     type Incoming<'a> = Incoming<'a>;
//     type Outcoming = Void;
//     type Precedent = Precedent;
//     fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
//         return async move {
//             todo!("Do after token creating");
//             // let user__id = match Extractor::<UserAccessToken>::extract(
//             //     &inner.environment_configuration.subject.encryption.private_key,
//             //     &incoming.user_access_token_signed,
//             // )? {
//             //     Extracted::Data {
//             //         user_access_token__id: _,
//             //         user__id: user__id_,
//             //         user_device__id: _,
//             //         user_access_token__expires_at: _,
//             //     } => user__id_,
//             //     Extracted::AlreadyExpired => return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired)),
//             //     Extracted::InUserAccessTokenBlackList => return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList))
//             // };
//             // if !Validator::<ChannelPublication1_Id>::is_valid(incoming.channel_publication1__id) {
//             //     return Result::Err(crate::new_invalid_argument!());
//             // }
//             // // let postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
//             // // let channel__owner = match Repository::<Postgresql<Channel>>::find_7(
//             // //     &postgresql_database_3_client,
//             // //     ChannelBy1 {
//             // //         channel__id: incoming.channel__id,
//             // //     },
//             // // )
//             // // .await?
//             // // {
//             // //     Option::Some(channel__owner_) => channel__owner_,
//             // //     Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NotFound))
//             // // };
//             // // if user__id != channel__owner {
//             // //     return Result::Ok(UnifiedReport::precedent(Precedent::User_IsNotChannelOwner));
//             // // }
//             // // let channel_publication1__created_at = Resolver::<UnixTime>::get_now_in_seconds();
//             // // let channel_publication1__id = Repository::<Postgresql<ChannelPublication1>>::create(
//             // //     &postgresql_database_3_client,
//             // //     ChannelPublication1Insert {
//             // //         channel__id: incoming.channel__id,
//             // //         channel_publication1__images_pathes: incoming.channel_publication1__images_pathes.as_slice(),
//             // //         channel_publication1__text: incoming.channel_publication1__text,
//             // //         channel_publication1__marks_quantity: 0,
//             // //         channel_publication1__viewing_quantity: 0,
//             // //         channel_publication1__created_at,
//             // //     },
//             // // ).await?;
//             // // return Result::Ok(
//             // //     UnifiedReport::target_filled(
//             // //         Outcoming {
//             // //             channel_publication1__id,
//             // //             channel_publication1__created_at,
//             // //         }
//             // //     )
//             // // );
//         };
//     }
// }
