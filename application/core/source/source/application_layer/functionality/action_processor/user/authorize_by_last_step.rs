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
                user::{
                    User,
                    User_ObfuscatedId,
                },
                user_access_refresh_token::{
                    UserAccessRefreshToken,
                    UserAccessRefreshToken_ExpiresAt,
                    UserAccessRefreshToken_ObfuscationValue,
                },
                user_access_token::{
                    UserAccessToken,
                    UserAccessToken_ExpiresAt,
                    UserAccessToken_ObfuscationValue,
                },
                user_authorization_token::{
                    UserAuthorizationToken,
                    UserAuthorizationToken_Value,
                    UserAuthorizationToken_WrongEnterTriesQuantity,
                },
                user_device::{
                    UserDevice,
                    UserDevice_Id,
                },
            },
            functionality::service::{
                encoder::Encoder,
                generator::Generator,
                validator::Validator,
            },
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
                        IsolationLevel,
                        Postgresql,
                        Resolver as Resolver_,
                        Transaction,
                        UserAccessRefreshTokenInsert,
                        UserAuthorizationTokenBy,
                        UserBy4,
                        UserDeviceInsert,
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
        action_processor_incoming_outcoming::action_processor::user::authorize_by_last_step::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::{
        future::Future,
        num::NonZero,
    },
};
pub struct AuthorizeByLastStep;
impl ActionProcessor_ for ActionProcessor<AuthorizeByLastStep> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_ObfuscatedId>::is_valid(incoming.user__obfuscated_id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserAuthorizationToken_Value>::is_valid(incoming.user_authorization_token__value)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let mut postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_authorization_token__value, mut user_authorization_token__wrong_enter_tries_quantity, user_authorization_token__expires_at) =
                match Repository::<Postgresql<UserAuthorizationToken>>::find_2(
                    &postgresql_client_database_2,
                    UserAuthorizationTokenBy {
                        user__obfuscated_id: incoming.user__obfuscated_id,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    Option::Some(values) => values,
                    Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken__NotFound)),
                };
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            if user_authorization_token__expires_at <= now {
                if !Repository::<Postgresql<UserAuthorizationToken>>::delete(
                    &postgresql_client_database_2,
                    UserAuthorizationTokenBy {
                        user__obfuscated_id: incoming.user__obfuscated_id,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken__AlreadyExpired));
            }
            if user_authorization_token__value != incoming.user_authorization_token__value {
                if user_authorization_token__wrong_enter_tries_quantity < UserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                    user_authorization_token__wrong_enter_tries_quantity += 1;
                }
                if user_authorization_token__wrong_enter_tries_quantity < UserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                    if !Repository::<Postgresql<UserAuthorizationToken>>::update_4(
                        &postgresql_client_database_2,
                        UserAuthorizationTokenBy {
                            user__obfuscated_id: incoming.user__obfuscated_id,
                            user_device__id: incoming.user_device__id,
                        },
                    )
                    .await?
                    {
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                } else {
                    if !Repository::<Postgresql<UserAuthorizationToken>>::delete(
                        &postgresql_client_database_2,
                        UserAuthorizationTokenBy {
                            user__obfuscated_id: incoming.user__obfuscated_id,
                            user_device__id: incoming.user_device__id,
                        },
                    )
                    .await?
                    {
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserAuthorizationToken__WrongValue {
                            user_authorization_token__wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            let postgresql_client_database_1 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            let user__id = match Repository::<Postgresql<User>>::find_7(
                &postgresql_client_database_1,
                UserBy4 {
                    user__obfuscated_id: incoming.user__obfuscated_id,
                },
            )
            .await?
            {
                Option::Some(user__id_) => user__id_,
                Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::User__NotFound)),
            };
            let user_access_token__obfuscation_value = Generator::<UserAccessToken_ObfuscationValue>::generate();
            let user_access_token__expires_at = Generator::<UserAccessToken_ExpiresAt>::generate(now)?;
            let user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?;
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_client_database_2,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let is_upserted = match Repository::<Postgresql<UserAccessRefreshToken>>::upsert(
                transaction.get_client(),
                UserAccessRefreshTokenInsert {
                    user__id,
                    user_device__id: incoming.user_device__id,
                    user_access_token__obfuscation_value,
                    user_access_refresh_token__obfuscation_value,
                    user_access_refresh_token__expires_at,
                    user_access_refresh_token__updated_at: now,
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
            if !is_upserted {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            let is_deleted = match Repository::<Postgresql<UserAuthorizationToken>>::delete(
                transaction.get_client(),
                UserAuthorizationTokenBy {
                    user__obfuscated_id: incoming.user__obfuscated_id,
                    user_device__id: incoming.user_device__id,
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
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            let private_key = &inner.environment_configuration.subject.encryption.private_key;
            let sended = Sended_::new(&raw const incoming as *const Self::Incoming<'static>);
            let (user_access_token_signed, user_access_refresh_token_signed) = crate::result_return_runtime!(
                TaskSpawner::spawn_rayon_task_processed(
                    move || -> _ {
                        let incoming_ = unsafe { sended.read_() };
                        return Ok(
                            (
                                Encoder::<UserAccessToken>::encode(
                                    private_key,
                                    user__id,
                                    incoming_.user_device__id,
                                    user_access_token__obfuscation_value,
                                    user_access_token__expires_at,
                                )?,
                                Encoder::<UserAccessRefreshToken>::encode(
                                    private_key,
                                    user__id,
                                    incoming_.user_device__id,
                                    user_access_token__obfuscation_value,
                                    user_access_refresh_token__obfuscation_value,
                                    user_access_refresh_token__expires_at,
                                    now,
                                )?,
                            ),
                        );
                    },
                ).await
            )?;
            let postgresql_connection_pool_database_1 = inner.postgresql_connection_pool_database_1.clone();
            let user_device__id = incoming.user_device__id.to_string();
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
                    let postgresql_connection_pool_database_1_ = postgresql_connection_pool_database_1.clone();
                    let user_device__id_ = user_device__id.clone();
                    return async move {
                        Repository::<Postgresql<UserDevice>>::create(
                            &crate::result_return_runtime!(postgresql_connection_pool_database_1_.get().await),
                            UserDeviceInsert {
                                user_device__id: user_device__id_.as_str(),
                                user__id,
                            },
                        )
                        .await?;
                        return Result::Ok(());
                    };
                },
            );
            let outcoming = Outcoming {
                user_access_token_signed,
                user_access_refresh_token_signed,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
