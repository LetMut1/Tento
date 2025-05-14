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
                    Channel_LinkedName,
                    Channel_Name,
                }, channel_token::{
                    ChannelToken,
                    ChannelToken_ExpiresAt,
                    ChannelToken_ObfuscationValue,
                }, quantity_limiter::{QuantityLimiter, QuantityLimiter_OwnedChannelsQuantity}, user_access_token::UserAccessToken
            },
            functionality::service::{
                encoder::Encoder, generator::Generator, validator::Validator
            },
        },
        infrastructure_layer::{
            data::{aggregate_error::AggregateError, sended::Sended_},
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy2, ChannelBy3, ChannelInsert, IsolationLevel, Postgresql, QuantityLimiterBy, QuantityLimiterInsert, QuantityLimiterUpdate, Resolver as Resolver_, Transaction
                    }, Repository
                },
                service::{resolver::{
                    Resolver,
                    UnixTime,
                }, task_spawner::TaskSpawner},
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel::create::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future,
};
pub struct Create;
impl ActionProcessor_ for ActionProcessor<Create> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            let private_key = &inner.environment_configuration.subject.encryption.private_key;
            let sended = Sended_::new(&raw const incoming as *const Self::Incoming<'static>);
            if let Option::Some(precedent) = crate::result_return_runtime!(
                TaskSpawner::spawn_rayon_task_processed(
                    move || -> Result<Option<Precedent>, AggregateError> {
                        let incoming_ = unsafe { sended.read_() };
                        if !Encoder::<UserAccessToken>::is_valid(
                            private_key,
                            &incoming_.user_access_token_signed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if incoming_.user_access_token_signed.user_access_token__expires_at <= now {
                            return Result::Ok(Option::Some(Precedent::UserAccessToken__AlreadyExpired));
                        }
                        if !Validator::<Channel_Name>::is_valid(incoming_.channel__name) {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if !Validator::<Channel_LinkedName>::is_valid(incoming_.channel__linked_name) {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        return Result::Ok(Option::None);
                    },
                ).await
            )? {
                return Result::Ok(UnifiedReport::precedent(precedent));
            }
            let mut postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            if Repository::<Postgresql<Channel>>::is_exist_1(
                &postgresql_client_database_3,
                ChannelBy2 {
                    channel__name: incoming.channel__name,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NameAlreadyExist));
            }
            if Repository::<Postgresql<Channel>>::is_exist_2(
                &postgresql_client_database_3,
                ChannelBy3 {
                    channel__linked_name: incoming.channel__linked_name,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__LinkedNameAlreadyExist));
            }
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
            let channel__id = match Repository::<Postgresql<Channel>>::create(
                transaction.get_client(),
                ChannelInsert {
                    channel__owner: incoming.user_access_token_signed.user__id,
                    channel__name: incoming.channel__name,
                    channel__linked_name: incoming.channel__linked_name,
                    channel__description: Option::None,
                    channel__access_modifier: incoming.channel__access_modifier,
                    channel__visability_modifier: incoming.channel__visability_modifier,
                    channel__cover_image_path: Option::None,
                    channel__background_image_path: Option::None,
                    channel__subscribers_quantity: 0,
                    channel__created_at: now,
                },
            )
            .await
            {
                Result::Ok(channel__id_) => channel__id_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            let channel__id_ = match channel__id {
                Option::Some(channel__id__) => channel__id__,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution)),
            };
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            let private_key = &inner.environment_configuration.subject.encryption.private_key;
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel_token_signed: crate::result_return_runtime!(
                            TaskSpawner::spawn_rayon_task_processed(
                                move || -> _ {
                                    return Encoder::<ChannelToken>::encode(
                                        private_key,
                                        incoming.user_access_token_signed.user__id,
                                        channel__id_,
                                        Generator::<ChannelToken_ObfuscationValue>::generate(),
                                        Generator::<ChannelToken_ExpiresAt>::generate(now)?,
                                        false,
                                        true,
                                    );
                                }
                            ).await
                        )?,
                    },
                ),
            );
        };
    }
}
