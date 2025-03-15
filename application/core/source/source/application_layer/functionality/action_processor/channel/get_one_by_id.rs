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
                    Channel_AccessModifier_,
                    Channel_Id,
                },
                channel_subscription::ChannelSubscription,
                channel_subscription_token::{
                    ChannelSubscriptionToken,
                    ChannelSubscriptionToken_ExpiresAt,
                },
                channel_token::ChannelToken,
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder,
                generator::Generator,
                validator::Validator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelBy1,
                        ChannelSubscriptionBy,
                        Postgresql,
                    },
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
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
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if incoming.user_access_token_signed.user_access_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken__AlreadyExpired));
            }
            if !Validator::<Channel_Id>::is_valid(incoming.channel__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let postgresql_database_3_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
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
                channel__obfuscation_value,
            ) = match Repository::<Postgresql<Channel>>::find_1(
                &postgresql_database_3_client,
                ChannelBy1 {
                    channel__id: incoming.channel__id,
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NotFound)),
            };
            if incoming.user_access_token_signed.user__id != channel__owner {
                match incoming.channel_token_hashed {
                    Option::Some(ref channel_token_hashed) => {
                        if !Encoder::<ChannelToken>::is_valid(
                            incoming.user_access_token_signed.user__id,
                            incoming.channel__id,
                            channel__obfuscation_value,
                            channel_token_hashed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if channel_token_hashed.channel_token__expires_at < now {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ChannelToken__AlreadyExpired));
                        }
                        if channel__access_modifier == Channel_AccessModifier_::Close as i16
                            && !Repository::<Postgresql<ChannelSubscription>>::is_exist(
                                &postgresql_database_3_client,
                                ChannelSubscriptionBy {
                                    user__id: incoming.user_access_token_signed.user__id,
                                    channel__id: incoming.channel__id,
                                },
                            )
                            .await?
                        {
                            return Result::Ok(UnifiedReport::precedent(Precedent::Channel__IsClose));
                        }
                    }
                    Option::None => {
                        if !Repository::<Postgresql<ChannelSubscription>>::is_exist(
                            &postgresql_database_3_client,
                            ChannelSubscriptionBy {
                                user__id: incoming.user_access_token_signed.user__id,
                                channel__id: incoming.channel__id,
                            },
                        )
                        .await?
                        {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ChannelToken__NotFound));
                        }
                    }
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
                user_is_channel_owner: incoming.user_access_token_signed.user__id == channel__owner,
                channel_subscription_token_hashed: Encoder::<ChannelSubscriptionToken>::encode(
                    incoming.user_access_token_signed.user__id,
                    incoming.channel__id,
                    channel__obfuscation_value,
                    Generator::<ChannelSubscriptionToken_ExpiresAt>::generate(now)?,
                )?,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
