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
                channel_subscription_token::{
                    ChannelSubscriptionToken_ExpiresAt,
                    ChannelSubscriptionToken,
                },
                user_access_token::UserAccessToken,
                channel_token::ChannelToken,
            },
            functionality::service::{
                extractor::{
                    Extracted,
                    Extractor,
                },
                validator::Validator,
                encoder::Encoder,
                generator::Generator,
            },
        },
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy1,
                        ChannelSubscriptionBy,
                        Postgresql,
                    },
                    Repository,
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
            let user__id = match Extractor::<UserAccessToken>::extract(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                Extracted::Data {
                    user_access_token__id: _,
                    user__id: user__id_,
                    user_device__id: _,
                    user_access_token__expires_at: _,
                } => user__id_,
                Extracted::AlreadyExpired => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken_AlreadyExpired));
                }
                Extracted::InUserAccessTokenBlackList => {
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
                channel__obfuscation_value,
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
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if user__id != channel__owner {
                match incoming.channel_token_hashed {
                    Option::Some(ref channel_token_hashed) => {
                        if !Encoder::<ChannelToken>::is_valid(
                            user__id,
                            incoming.channel__id,
                            channel__obfuscation_value,
                            channel_token_hashed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if channel_token_hashed.channel_token__expires_at < now {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ChannelToken_AlreadyExpired));
                        }
                        if channel__access_modifier == Channel_AccessModifier::Close as i16
                        && !Repository::<Postgresql<ChannelSubscription>>::is_exist(
                            &postgresql_database_1_client,
                            ChannelSubscriptionBy {
                                user__id,
                                channel__id: incoming.channel__id,
                            },
                        )
                        .await? {
                            return Result::Ok(UnifiedReport::precedent(Precedent::Channel_IsClose));
                        }
                    }
                    Option::None => {
                        if !Repository::<Postgresql<ChannelSubscription>>::is_exist(
                            &postgresql_database_1_client,
                            ChannelSubscriptionBy {
                                user__id,
                                channel__id: incoming.channel__id,
                            },
                        )
                        .await?
                        {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ChannelToken_NotFound));
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
                user_is_channel_owner: user__id == channel__owner,
                channel_subscription_token_hashed: Encoder::<ChannelSubscriptionToken>::encode(
                    user__id,
                    incoming.channel__id,
                    channel__obfuscation_value,
                    Generator::<ChannelSubscriptionToken_ExpiresAt>::generate(now)?,
                )?,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
