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
                    User_Id,
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
            data::aggregate_error::AggregateError,
            functionality::{
                repository::{
                    Repository,
                    postgresql::{
                        IsolationLevel,
                        Postgresql,
                        Resolver as Resolver_,
                        Transaction,
                        UserAccessRefreshTokenBy2,
                        UserAccessRefreshTokenInsert,
                        UserAccessRefreshTokenUpdate,
                        UserAuthorizationTokenBy,
                        UserBy3,
                        UserDeviceInsert,
                    },
                },
                service::{
                    resolver::{
                        Resolver,
                        UnixTime,
                    },
                    spawner::{
                        Spawner,
                        TokioNonBlockingTask,
                    },
                },
            },
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::authorize_by_last_step::{
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
pub struct UserAuthorization_AuthorizeByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_AuthorizeByLastStep> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserAuthorizationToken_Value>::is_valid(incoming.user_authorization_token__value)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let mut postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (user_authorization_token__value, mut user_authorization_token__wrong_enter_tries_quantity, user_authorization_token__expires_at) =
                match Repository::<Postgresql<UserAuthorizationToken>>::find_2(
                    &postgresql_database_2_client,
                    UserAuthorizationTokenBy {
                        user__id: incoming.user__id,
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
                    &postgresql_database_2_client,
                    UserAuthorizationTokenBy {
                        user__id: incoming.user__id,
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
                        &postgresql_database_2_client,
                        UserAuthorizationTokenBy {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id,
                        },
                    )
                    .await?
                    {
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                } else {
                    if !Repository::<Postgresql<UserAuthorizationToken>>::delete(
                        &postgresql_database_2_client,
                        UserAuthorizationTokenBy {
                            user__id: incoming.user__id,
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
            let postgresql_database_1_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            if !Repository::<Postgresql<User>>::is_exist_3(
                &postgresql_database_1_client,
                UserBy3 {
                    user__id: incoming.user__id,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User__NotFound));
            }
            let user_access_token__obfuscation_value = Generator::<UserAccessToken_ObfuscationValue>::generate();
            let user_access_token__expires_at = Generator::<UserAccessToken_ExpiresAt>::generate(now)?;
            let user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?;
            let user_access_refresh_token__updated_at = now;
            let is_exist = Repository::<Postgresql<UserAccessRefreshToken>>::is_exist(
                &postgresql_database_2_client,
                UserAccessRefreshTokenBy2 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id,
                },
            )
            .await?;
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_2_client,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            if is_exist {
                let is_updated = match Repository::<Postgresql<UserAccessRefreshToken>>::update(
                    transaction.get_client(),
                    UserAccessRefreshTokenUpdate {
                        user_access_token__obfuscation_value,
                        user_access_refresh_token__obfuscation_value,
                        user_access_refresh_token__expires_at,
                        user_access_refresh_token__updated_at,
                    },
                    UserAccessRefreshTokenBy2 {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id,
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
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
            } else {
                let is_created = match Repository::<Postgresql<UserAccessRefreshToken>>::create(
                    transaction.get_client(),
                    UserAccessRefreshTokenInsert {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id,
                        user_access_token__obfuscation_value,
                        user_access_refresh_token__obfuscation_value,
                        user_access_refresh_token__expires_at,
                        user_access_refresh_token__updated_at,
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
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
            };
            let is_deleted = match Repository::<Postgresql<UserAuthorizationToken>>::delete(
                transaction.get_client(),
                UserAuthorizationTokenBy {
                    user__id: incoming.user__id,
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
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            let user_access_token_signed = Encoder::<UserAccessToken>::encode(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user__id,
                incoming.user_device__id,
                user_access_token__obfuscation_value,
                user_access_token__expires_at,
            )?;
            let user_access_refresh_token_signed = Encoder::<UserAccessRefreshToken>::encode(
                &inner.environment_configuration.subject.encryption.private_key,
                incoming.user__id,
                incoming.user_device__id,
                user_access_token__obfuscation_value,
                user_access_refresh_token__obfuscation_value,
                user_access_refresh_token__expires_at,
                user_access_refresh_token__updated_at,
            )?;
            let postgresql_connection_pool_database_1 = inner.postgresql_connection_pool_database_1.clone();
            let user_device__id = incoming.user_device__id.to_string();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY));
                    '_a: for quantity in 1..=BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY {
                        interval.tick().await;
                        match Repository::<Postgresql<UserDevice>>::create(
                            &crate::result_return_runtime!(postgresql_connection_pool_database_1.get().await),
                            UserDeviceInsert {
                                user_device__id: user_device__id.as_str(),
                                user__id: incoming.user__id,
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
            let outcoming = Outcoming {
                user_access_token_signed,
                user_access_refresh_token_signed,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
