use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel::Channel, channel_subscription::ChannelSubscription, channel_token::{ChannelToken, ChannelToken_ExpiresAt, ChannelToken_ObfuscationValue}, user_access_token::UserAccessToken
            },
            functionality::service::{encoder::Encoder, generator::Generator},
        },
        infrastructure_layer::{
            data::{aggregate_error::AggregateError, sended::Sended_},
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy1,
                        ChannelSubscriptionInsert,
                        IsolationLevel,
                        Postgresql,
                        Resolver as Resolver_,
                        Transaction,
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
        action_processor_incoming_outcoming::action_processor::channel_subscription::create::{
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
                        if !Encoder::<ChannelToken>::is_valid(
                            private_key,
                            incoming_.user_access_token_signed.user__id,
                            &incoming_.channel_token_signed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if incoming_.channel_token_signed.channel_token__expires_at <= now {
                            return Result::Ok(Option::Some(Precedent::ChannelToken__AlreadyExpired));
                        }
                        return Result::Ok(Option::None);
                    },
                ).await
            )? {
                return Result::Ok(UnifiedReport::precedent(precedent));
            }
            if incoming.channel_token_signed.channel_token__is_user_the_channel_owner {
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__UserIsOwner));
            }
            if incoming.channel_token_signed.channel_token__is_user_the_channel_subscriber {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelSubscription__AlreadyExist));
            }
            let mut postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_client_database_3,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let is_created = match Repository::<Postgresql<ChannelSubscription>>::create(
                transaction.get_client(),
                ChannelSubscriptionInsert {
                    user__id: incoming.user_access_token_signed.user__id,
                    channel__id: incoming.channel_token_signed.channel__id,
                    channel_subscription__created_at: now,
                },
            )
            .await
            {
                Result::Ok(is_created_) => is_created_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_created {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelSubscription__AlreadyExist));
            }
            let is_updated = match Repository::<Postgresql<Channel>>::update_1(
                transaction.get_client(),
                ChannelBy1 {
                    channel__id: incoming.channel_token_signed.channel__id,
                },
            )
            .await
            {
                Result::Ok(is_updated_) => is_updated_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_updated {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NotFound));
            }
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
                                        incoming.channel_token_signed.channel__id,
                                        Generator::<ChannelToken_ObfuscationValue>::generate(),
                                        Generator::<ChannelToken_ExpiresAt>::generate(now)?,
                                        true,
                                        incoming.channel_token_signed.channel_token__is_user_the_channel_owner,
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
