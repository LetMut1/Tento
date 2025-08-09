use {
    crate::{
        BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY,
        BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY,
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                channel_publication1::ChannelPublication1,
                channel_publication1_marked_view::{
                    ChannelPublication1MarkedView,
                    ChannelPublication1MarkedView_MarkedAt,
                },
                channel_publication1_token::ChannelPublication1Token,
                channel_token::ChannelToken,
                user_access_token::UserAccessToken,
            },
            functionality::service::encoder::Encoder,
        },
        infrastructure_layer::{
            data::{
                aggregate_error::AggregateError,
                sended::Sended_,
            },
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        ChannelPublication1By1,
                        ChannelPublication1MarkedViewInsert,
                        Postgresql,
                    },
                },
                service::{
                    resolver::{
                        Resolver,
                        UnixTime,
                    },
                    task_spawner::{
                        RepeatableForError,
                        TaskSpawner,
                    },
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::channel_publication1_marked_view::create_view::{
            Incoming,
            Precedent,
        },
        unified_report::UnifiedReport,
        void::Void,
    },
    std::{
        future::Future,
        num::NonZero,
    },
};
pub struct CreateView;
impl ActionProcessor_ for ActionProcessor<CreateView> {
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
                        if !Encoder::<ChannelPublication1Token>::is_valid(
                            private_key,
                            incoming_.user_access_token_signed.user__id,
                            incoming_.channel_token_signed.channel__id,
                            &incoming_.channel_publication1_token_signed,
                        )? {
                            return Result::Err(crate::new_invalid_argument!());
                        }
                        if incoming_.channel_publication1_token_signed.channel_publication1_token__expires_at <= now {
                            return Result::Ok(Option::Some(Precedent::ChannelPublication1Token__AlreadyExpired));
                        }
                        return Result::Ok(Option::None);
                    },
                ).await
            )? {
                return Result::Ok(UnifiedReport::precedent(precedent));
            }
            let mut postgresql_connection_pool_database_3 = inner.postgresql_connection_pool_database_3.clone();
            TaskSpawner::spawn_tokio_non_blocking_task_into_background_repeatable(
                RepeatableForError {
                    quantity: unsafe {
                        static_assertions::const_assert!(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY > 0);
                        NonZero::<usize>::new_unchecked(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY)
                    },
                    interval_seconds_quantity: unsafe {
                        static_assertions::const_assert!(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY > 0);
                        NonZero::<u64>::new_unchecked(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY)
                    },
                },
                move || -> _ {
                    let postgresql_connection_pool_database_3_ = postgresql_connection_pool_database_3.clone();
                    return async move {
                        Repository::<Postgresql<ChannelPublication1MarkedView>>::create_1(
                            &crate::result_return_runtime!(postgresql_connection_pool_database_3_.get().await),
                            ChannelPublication1MarkedViewInsert {
                                user__id: incoming.user_access_token_signed.user__id,
                                channel_publication1__id: incoming.channel_publication1_token_signed.channel_publication1__id,
                                channel_publication1_marked_view__marked_at: ChannelPublication1MarkedView_MarkedAt::VALUE_FOR_INDICATION_OF_MARK_ABSENCE as i64,
                                channel_publication1_marked_view__created_at: now,
                            },
                        )
                        .await?;
                        return Result::Ok(());
                    };
                },
            );
            postgresql_connection_pool_database_3 = inner.postgresql_connection_pool_database_3.clone();
            TaskSpawner::spawn_tokio_non_blocking_task_into_background_repeatable(
                RepeatableForError {
                    quantity: unsafe {
                        static_assertions::const_assert!(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY > 0);
                        NonZero::<usize>::new_unchecked(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY)
                    },
                    interval_seconds_quantity: unsafe {
                        static_assertions::const_assert!(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY > 0);
                        NonZero::<u64>::new_unchecked(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY)
                    },
                },
                move || -> _ {
                    let postgresql_connection_pool_database_3_ = postgresql_connection_pool_database_3.clone();
                    return async move {
                        Repository::<Postgresql<ChannelPublication1>>::update_3(
                            &crate::result_return_runtime!(postgresql_connection_pool_database_3_.get().await),
                            ChannelPublication1By1 {
                                channel_publication1__id: incoming.channel_publication1_token_signed.channel_publication1__id,
                            },
                        )
                        .await?;
                        return Result::Ok(());
                    };
                },
            );
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
