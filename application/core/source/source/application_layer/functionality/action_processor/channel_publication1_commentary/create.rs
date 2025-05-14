use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        }, domain_layer::{
            data::entity::{
                channel_publication1::ChannelPublication1,
                channel_publication1_commentary::{
                    ChannelPublication1Commentary,
                    ChannelPublication1Commentary_Text,
                },
                channel_publication1_token::ChannelPublication1Token,
                user_access_token::UserAccessToken,
            },
            functionality::service::{
                encoder::Encoder,
                validator::Validator,
            },
        }, infrastructure_layer::{
            data::{aggregate_error::AggregateError, sended::Sended_},
            functionality::{
                repository::{
                    postgresql::{
                        ChannelPublication1By1,
                        ChannelPublication1CommentaryInsert,
                        Postgresql,
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
        action_processor_incoming_outcoming::action_processor::channel_publication1_commentary::create::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::{
        future::Future,
        time::Duration,
    },
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
                        if !Validator::<ChannelPublication1Commentary_Text>::is_valid(incoming_.channel_publication1_commentary__text) {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if !Encoder::<ChannelPublication1Token>::is_valid(
                            private_key,
                            incoming_.user_access_token_signed.user__id,
                            &incoming_.channel_publication1_token_signed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if incoming_.channel_publication1_token_signed.channel_publication1_token__expires_at < now {
                            return Result::Ok(Option::Some(Precedent::ChannelPublication1Token__AlreadyExpired));
                        }
                        return Result::Ok(Option::None);
                    },
                ).await
            )? {
                return Result::Ok(UnifiedReport::precedent(precedent));
            }
            let channel_publication1_commentary__id = match Repository::<Postgresql<ChannelPublication1Commentary>>::create(
                &crate::result_return_runtime!(inner.postgresql_connection_pool_database_4.get().await),
                ChannelPublication1CommentaryInsert {
                    channel_publication1_commentary__author: incoming.user_access_token_signed.user__id,
                    channel_publication1__id: incoming.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_commentary__text: incoming.channel_publication1_commentary__text,
                    channel_publication1_commentary__marks_quantity: 0,
                    channel_publication1_commentary__created_at: now,
                },
            )
            .await?
            {
                Option::Some(channel_publication1_commentary__id_) => channel_publication1_commentary__id_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution)),
            };
            let postgresql_connection_pool_database_3 = inner.postgresql_connection_pool_database_3.clone();
            TaskSpawner::spawn_tokio_non_blocking_task_into_background(
                async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY));
                    '_a: for quantity in 1..=BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY {
                        interval.tick().await;
                        match Repository::<Postgresql<ChannelPublication1>>::update_4(
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
            return Result::Ok(
                UnifiedReport::target_filled(
                    Outcoming {
                        channel_publication1_commentary__id,
                        channel_publication1_commentary__created_at: now,
                    },
                ),
            );
        };
    }
}
