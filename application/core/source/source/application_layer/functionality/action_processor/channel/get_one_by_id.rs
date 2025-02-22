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
                    ChannelSubscriptionBy1,
                    Postgresql,
                },
                Repository,
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::get_one_by_id::{
            Incoming,
            Outcoming,
            Precedent,
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
            let (
                channel__owner,
                channel__name,
                channel__linked_name,
                channel__description,
                channel__access_modifier,
                channel__visability_modifier,
                channel__orientation,
                channel__cover_image_path,
                channel__background_image_path,
                channel__subscribers_quantity,
                channel__marks_quantity,
                channel__viewing_quantity,
            ) = match Repository::<Postgresql<Channel>>::find_1(
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
            if Channel_AccessModifier::Close as i16 == channel__access_modifier {
                let is_exist = Repository::<Postgresql<ChannelSubscription>>::is_exist_1(
                    &postgresql_database_1_client,
                    ChannelSubscriptionBy1 {
                        user__id: user_access_token.user__id,
                        channel__id: incoming.channel__id,
                    },
                )
                .await?;
                if !is_exist && user_access_token.user__id != channel__owner {
                    return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
                }
            }
            let outcoming = Outcoming {
                channel__name,
                channel__linked_name,
                channel__description,
                channel__access_modifier,
                channel__visability_modifier,
                channel__orientation,
                channel__cover_image_path,
                channel__background_image_path,
                channel__subscribers_quantity,
                channel__marks_quantity,
                channel__viewing_quantity,
                user_is_channel_owner: user_access_token.user__id == channel__owner,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
