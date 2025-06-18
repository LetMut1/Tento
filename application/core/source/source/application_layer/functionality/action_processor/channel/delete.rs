use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel::Channel, channel_delayed_deletion::{ChannelDelayedDeletion, ChannelDelayedDeletion_CanBeDeletedFrom}, channel_token::ChannelToken, quantity_limiter::QuantityLimiter, user_access_token::UserAccessToken
            },
            functionality::service::{
                encoder::Encoder, generator::Generator,
            },
        },
        infrastructure_layer::{
            data::{aggregate_error::AggregateError, sended::Sended_},
            functionality::{
                repository::{
                    postgresql::{
                        ChannelBy7, ChannelDelayedDeletionInsert, IsolationLevel, Postgresql, QuantityLimiterBy, Resolver as Resolver_, Transaction
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
        action_processor_incoming_outcoming::action_processor::channel::delete::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::future::Future,
};
pub struct Delete;
impl ActionProcessor_ for ActionProcessor<Delete> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
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
            if !incoming.channel_token_signed.channel_token__is_user_the_channel_owner {
                return Result::Err(crate::new_invalid_argument!());
            }
            let mut postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_client_database_3,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let is_updated = match Repository::<Postgresql<QuantityLimiter>>::update_2(
                transaction.get_client(),
                QuantityLimiterBy {
                    user__id: incoming.user_access_token_signed.user__id,
                }
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
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            let is_deleted = match Repository::<Postgresql<Channel>>::delete(
                transaction.get_client(),
                ChannelBy7 {
                    channel__id: incoming.channel_token_signed.channel__id,
                    channel__owner: incoming.user_access_token_signed.user__id,
                }
            )
            .await
            {
                Result::Ok(is_deleted_) => is_deleted_,
                Result::Err(aggregate_error) => {
                    Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                    return Result::Err(aggregate_error);
                }
            };
            if !is_deleted {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NotFound));
            }
            let is_created = match Repository::<Postgresql<ChannelDelayedDeletion>>::create(
                transaction.get_client(),
                ChannelDelayedDeletionInsert {
                    channel__id: incoming.channel_token_signed.channel__id,
                    channel_delayed_deletion__can_be_deleted_from: Generator::<ChannelDelayedDeletion_CanBeDeletedFrom>::generate(now)?,
                    channel_delayed_deletion__created_at: now,
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
                return Result::Ok(UnifiedReport::precedent(Precedent::Channel__NotFound));
            }
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            return Result::Ok(UnifiedReport::target_empty());
        }
    }
}
