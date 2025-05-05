use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel_publication1::{
                    ChannelPublication1,
                    ChannelPublication1_ImagesPathes,
                    ChannelPublication1_Text,
                }, channel_publication1_token::{
                    ChannelPublication1Token,
                    ChannelPublication1Token_ExpiresAt,
                    ChannelPublication1Token_ObfuscationValue,
                }, channel_token::ChannelToken, quantity_limiter::{QuantityLimiter, QuantityLimiter_OwnedChannelsQuantity}, user_access_token::UserAccessToken
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
                    postgresql::{
                        ChannelPublication1Insert, IsolationLevel, Postgresql, QuantityLimiterBy, QuantityLimiterInsert, QuantityLimiterUpdate, Resolver as Resolver_, Transaction
                    }, Repository,
                },
                service::resolver::{
                    Resolver,
                    UnixTime,
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1::create::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct ChannelPublication1_Create;
impl ActionProcessor_ for ActionProcessor<ChannelPublication1_Create> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            if !Encoder::<UserAccessToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                &incoming.user_access_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.user_access_token_signed.user_access_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAccessToken__AlreadyExpired));
            }
            if !Encoder::<ChannelToken>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user_access_token_signed.user__id,
                &incoming.channel_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.channel_token_signed.channel_token__expires_at <= now {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelToken__AlreadyExpired));
            }
            if incoming.channel_publication1__text.is_none() && incoming.channel_publication1__images_pathes.is_empty() {
                return Result::Err(crate::new_invalid_argument!());
            }
            if let Option::Some(channel_publication1__text) = incoming.channel_publication1__text {
                if !Validator::<ChannelPublication1_Text>::is_valid(channel_publication1__text) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            if !incoming.channel_publication1__images_pathes.is_empty() {
                if !Validator::<ChannelPublication1_ImagesPathes>::is_valid(incoming.channel_publication1__images_pathes.as_slice()) {
                    return Result::Err(crate::new_invalid_argument!());
                }
            }
            if !incoming.channel_token_signed.channel_token__is_user_the_channel_owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::User__IsNotChannelOwner));
            }
            let mut postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let transaction = match Repository::<Postgresql<QuantityLimiter>>::find(
                &postgresql_client_database_3,
                QuantityLimiterBy {
                    user__id: incoming.user_access_token_signed.user__id,
                }
            )
            .await?
            {
                Option::Some(quantity_limiter__owned_channels_quantity) => {
                    if quantity_limiter__owned_channels_quantity >= QuantityLimiter_OwnedChannelsQuantity::MAXIMUM_VALUE {
                        return Result::Ok(UnifiedReport::precedent(Precedent::QuantityLimiter__ExceededOwnedChannelsQuantity));
                    }
                    let transaction_ = Resolver_::<Transaction<'_>>::start(
                        &mut postgresql_client_database_3,
                        IsolationLevel::ReadCommitted,
                    )
                    .await?;
                    let is_updated = match Repository::<Postgresql<QuantityLimiter>>::update_1(
                        transaction_.get_client(),
                        QuantityLimiterUpdate {
                            quantity_limiter__owned_channels_quantity___maximum_value: QuantityLimiter_OwnedChannelsQuantity::MAXIMUM_VALUE,
                        },
                        QuantityLimiterBy {
                            user__id: incoming.user_access_token_signed.user__id,
                        }
                    )
                    .await
                    {
                        Result::Ok(is_updated_) => is_updated_,
                        Result::Err(aggregate_error) => {
                            Resolver_::<Transaction<'_>>::rollback(transaction_).await?;
                            return Result::Err(aggregate_error);
                        }
                    };
                    if !is_updated {
                        Resolver_::<Transaction<'_>>::rollback(transaction_).await?;
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                    transaction_
                },
                Option::None => {
                    let transaction_ = Resolver_::<Transaction<'_>>::start(
                        &mut postgresql_client_database_3,
                        IsolationLevel::ReadCommitted,
                    )
                    .await?;
                    let is_created = match Repository::<Postgresql<QuantityLimiter>>::create(
                        transaction_.get_client(),
                        QuantityLimiterInsert {
                            user__id: incoming.user_access_token_signed.user__id,
                            quantity_limiter__owned_channels_quantity: 1,
                            quantity_limiter__created_at: now,
                        }
                    )
                    .await
                    {
                        Result::Ok(is_created_) => is_created_,
                        Result::Err(aggregate_error) => {
                            Resolver_::<Transaction<'_>>::rollback(transaction_).await?;
                            return Result::Err(aggregate_error);
                        }
                    };
                    if !is_created {
                        Resolver_::<Transaction<'_>>::rollback(transaction_).await?;
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                    transaction_
                }
            };
            let channel_publication1__id = match Repository::<Postgresql<ChannelPublication1>>::create(
                transaction.get_client(),
                ChannelPublication1Insert {
                    channel__id: incoming.channel_token_signed.channel__id,
                    channel_publication1__images_pathes: incoming.channel_publication1__images_pathes.as_slice(),
                    channel_publication1__text: incoming.channel_publication1__text,
                    channel_publication1__commentaries_quantity: 0,
                    channel_publication1__marks_quantity: 0,
                    channel_publication1__view_quantity: 0,
                    channel_publication1__created_at: now,
                    channel_publication1__can_be_deleted_from: 0,
                },
            )
            .await
            {
                Result::Ok(channel_publication1__id__) => channel_publication1__id__,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            let channel_publication1__id_ = match channel_publication1__id {
                Some(channel_publication1__id__) => channel_publication1__id__,
                None => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
            };
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel_publication1__created_at: now,
                        channel_publication1_token_signed: Encoder::<ChannelPublication1Token>::encode(
                            &inner.environment_configuration.subject.encryption.private_key,
                            incoming.user_access_token_signed.user__id,
                            incoming.channel_token_signed.channel__id,
                            channel_publication1__id_,
                            Generator::<ChannelPublication1Token_ObfuscationValue>::generate(),
                            Generator::<ChannelPublication1Token_ExpiresAt>::generate(now)?,
                        )?,
                    },
                ),
            );
        };
    }
}
