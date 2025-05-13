use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        }, domain_layer::{
            data::entity::{
                channel_publication1::ChannelPublication1, channel_publication1_commentary::{
                    ChannelPublication1Commentary,
                    ChannelPublication1Commentary_Id,
                }, channel_publication1_commentary_delayed_deletion::{ChannelPublication1CommentaryDelayedDeletion, ChannelPublication1CommentaryDelayedDeletion_CanBeDeletedFrom}, channel_publication1_token::ChannelPublication1Token, user_access_token::UserAccessToken
            },
            functionality::service::{
                encoder::Encoder,
                generator::Generator,
                validator::Validator,
            },
        }, infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    postgresql::{
                        ChannelPublication1By1, ChannelPublication1CommentaryBy, ChannelPublication1CommentaryDelayedDeletionInsert, IsolationLevel, Postgresql, Resolver as Resolver_, Transaction,
                    }, Repository
                },
                service::{
                    resolver::{
                        Resolver,
                        UnixTime,
                    },
                    task_spawner::TaskSpawner,
                },
            },
        }, BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY, BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1_commentary::delete::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::{
        future::Future,
        time::Duration,
    },
};
pub struct Delete;
impl ActionProcessor_ for ActionProcessor<Delete> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Void;
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
            if !Validator::<ChannelPublication1Commentary_Id>::is_valid(incoming.channel_publication1_commentary__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Encoder::<ChannelPublication1Token>::is_valid(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user_access_token_signed.user__id,
                &incoming.channel_publication1_token_signed,
            )? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if incoming.channel_publication1_token_signed.channel_publication1_token__expires_at < now {
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1Token__AlreadyExpired));
            }
            let mut postgresql_client_database_4 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_4.get().await);
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_client_database_4,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let is_deleted = match Repository::<Postgresql<ChannelPublication1Commentary>>::delete(
                transaction.get_client(),
                ChannelPublication1CommentaryBy {
                    channel_publication1_commentary__id: incoming.channel_publication1_commentary__id,
                    channel_publication1_commentary__author: incoming.user_access_token_signed.user__id,
                },
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
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1Commentary__NotFound));
            }
            let is_created = match Repository::<Postgresql<ChannelPublication1CommentaryDelayedDeletion>>::create(
                transaction.get_client(),
                ChannelPublication1CommentaryDelayedDeletionInsert {
                    channel_publication1_commentary__id: incoming.channel_publication1_commentary__id,
                    channel_publication1_commentary_delayed_deletion__can_be_deleted_from: Generator::<ChannelPublication1CommentaryDelayedDeletion_CanBeDeletedFrom>::generate(now)?,
                    channel_publication1_commentary_delayed_deletion__created_at: now,
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
                return Result::Ok(UnifiedReport::precedent(Precedent::ChannelPublication1Commentary__NotFound));
            }
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            let postgresql_connection_pool_database_3 = inner.postgresql_connection_pool_database_3.clone();
            TaskSpawner::spawn_tokio_non_blocking_task_into_background(
                async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY));
                    '_a: for quantity in 1..=BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY {
                        interval.tick().await;
                        match Repository::<Postgresql<ChannelPublication1>>::update_5(
                            &crate::result_return_runtime!(postgresql_connection_pool_database_3.get().await),
                            ChannelPublication1By1 {
                                channel_publication1__id: incoming.channel_publication1_token_signed.channel_publication1__id,
                            },
                        )
                        .await
                        {
                            Ok(_) => return Result::Ok(()),
                            Err(aggregate_error) => {
                                if quantity == BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY {
                                    return Err(aggregate_error);
                                }
                            }
                        }
                    }
                    return Result::Ok(());
                },
            );
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
