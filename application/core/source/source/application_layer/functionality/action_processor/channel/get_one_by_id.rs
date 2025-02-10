use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel::{
                    Channel,
                    Channel_AccessModifier,
                    Channel_Id,
                },
                channel_inner_link::ChannelInnerLink,
                channel_outer_link::ChannelOuterLink,
                channel_subscription::ChannelSubscription,
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                extractor::{
                    Extracted,
                    Extractor,
                },
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::repository::{
                postgresql::{
                    ChannelBy1,
                    ChannelInnerLinkBy1,
                    ChannelOuterLinkBy1,
                    ChannelSubscriptionBy1,
                    Postgresql,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::{
            action_processor::channel::get_one_by_id::{
                Incoming,
                Outcoming,
                Precedent,
            },
            Channel2,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Channel_GetOneById;
impl ActionProcessor_ for ActionProcessor<Channel_GetOneById> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let user_access_token = match Extractor::<UserAccessToken<'_>>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_encoded,
            )? {
                Extracted::UserAccessToken {
                    user_access_token: user_access_token_,
                } => user_access_token_,
                Extracted::UserAccessTokenAlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::UserAccessTokenInUserAccessTokenBlackList => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_InUserAccessTokenBlackList));
                }
            };
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_1_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            let channel = match Repository::<Postgresql<Channel<'_>>>::find_1(
                &postgresql_database_1_client,
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                Option::Some(channel_) => channel_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel_NotFound));
                }
            };
            if const { Channel_AccessModifier::Close as i16 } == channel.access_modifier {
                let is_exist = Repository::<Postgresql<ChannelSubscription>>::is_exist_1(
                    &postgresql_database_1_client,
                    ChannelSubscriptionBy1 {
                        user__id: user_access_token.user__id,
                        channel__id: channel.id,
                    },
                )
                .await?;
                if !is_exist && user_access_token.user__id != channel.owner {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
                }
            }
            let channel_inner_links = Repository::<Postgresql<ChannelInnerLink>>::find_1(
                &postgresql_database_1_client,
                ChannelInnerLinkBy1 {
                    channel_inner_link__from: channel.id,
                },
                ChannelInnerLink::MAXIMUM_QUANTITY,
            )
            .await?;
            let channel_outer_links = Repository::<Postgresql<ChannelOuterLink>>::find_1(
                &postgresql_database_1_client,
                ChannelOuterLinkBy1 {
                    channel_outer_link__from: channel.id,
                },
                ChannelOuterLink::MAXIMUM_QUANTITY,
            )
            .await?;
            let channel = Channel2 {
                channel__owner: channel.owner,
                channel__name: channel.name.into_owned(),
                channel__linked_name: channel.linked_name,
                channel__description: channel.description,
                channel__access_modifier: channel.access_modifier,
                channel__visability_modifier: channel.visability_modifier,
                channel__orientation: channel.orientation,
                channel__cover_image_path: channel.cover_image_path,
                channel__background_image_path: channel.background_image_path,
                channel__subscribers_quantity: channel.subscribers_quantity,
                channel__marks_quantity: channel.marks_quantity,
                channel__viewing_quantity: channel.viewing_quantity,
            };
            let outcoming = Outcoming {
                channel,
                channel_inner_links,
                channel_outer_links,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
