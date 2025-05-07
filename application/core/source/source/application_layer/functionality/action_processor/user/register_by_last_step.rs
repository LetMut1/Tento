use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        }, domain_layer::{
            data::entity::{
                user::{
                    User,
                    User_Email,
                    User_Nickname,
                    User_Password,
                }, user_access_refresh_token::{
                    UserAccessRefreshToken,
                    UserAccessRefreshToken_ExpiresAt,
                    UserAccessRefreshToken_ObfuscationValue,
                }, user_access_token::{
                    UserAccessToken,
                    UserAccessToken_ExpiresAt,
                    UserAccessToken_ObfuscationValue,
                }, user_device::{
                    UserDevice,
                    UserDevice_Id,
                }, user_registration_token::{
                    UserRegistrationToken,
                    UserRegistrationToken_Value,
                    UserRegistrationToken_WrongEnterTriesQuantity,
                }
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
                        IsolationLevel, Postgresql, Resolver as Resolver_, Transaction, UserAccessRefreshTokenInsert, UserBy1, UserBy2, UserBy3, UserDeviceInsert, UserInsert2, UserRegistrationTokenBy
                    }, Repository
                },
                service::{
                    resolver::{
                        Resolver,
                        UnixTime,
                    },
                    spawner::{
                        Spawner,
                        TokioBlockingTask,
                        TokioNonBlockingTask,
                    },
                },
            },
        }, BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY, BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user::register_by_last_step::{
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
pub struct RegisterByLastStep;
impl ActionProcessor_ for ActionProcessor<RegisterByLastStep> {
    type Incoming<'a> = Incoming<'a>;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming<'a>) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Password>::is_valid(
                incoming.user__password,
                incoming.user__email,
                incoming.user__nickname,
            ) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Nickname>::is_valid(incoming.user__nickname) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Email>::is_valid(incoming.user__email)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserRegistrationToken_Value>::is_valid(incoming.user_registration_token__value)? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            {
                let postgresql_client_database_1 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
                if Repository::<Postgresql<User>>::is_exist_1(
                    &postgresql_client_database_1,
                    UserBy1 {
                        user__nickname: incoming.user__nickname,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User__NicknameAlreadyExist));
                }
                if Repository::<Postgresql<User>>::is_exist_2(
                    &postgresql_client_database_1,
                    UserBy2 {
                        user__email: incoming.user__email,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User__EmailAlreadyExist));
                }
            }
            let now = Resolver::<UnixTime>::get_now_in_microseconds();
            {
                let postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
                let (
                    user_registration_token__value,
                    mut user_registration_token__wrong_enter_tries_quantity,
                    user_registration_token__is_approved,
                    user_registration_token__expires_at,
                ) = match Repository::<Postgresql<UserRegistrationToken>>::find_2(
                    &postgresql_client_database_2,
                    UserRegistrationTokenBy {
                        user__email: incoming.user__email,
                        user_device__id: incoming.user_device__id,
                    },
                )
                .await?
                {
                    Option::Some(values) => values,
                    Option::None => return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__NotFound)),
                };
                if user_registration_token__expires_at <= now {
                    if !Repository::<Postgresql<UserRegistrationToken>>::delete(
                        &postgresql_client_database_2,
                        UserRegistrationTokenBy {
                            user__email: incoming.user__email,
                            user_device__id: incoming.user_device__id,
                        },
                    )
                    .await?
                    {
                        return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                    }
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__AlreadyExpired));
                }
                if !user_registration_token__is_approved {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__IsNotApproved));
                }
                if user_registration_token__value != incoming.user_registration_token__value {
                    if user_registration_token__wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                        user_registration_token__wrong_enter_tries_quantity += 1;
                    }
                    if user_registration_token__wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                        if !Repository::<Postgresql<UserRegistrationToken>>::update_4(
                            &postgresql_client_database_2,
                            UserRegistrationTokenBy {
                                user__email: incoming.user__email,
                                user_device__id: incoming.user_device__id,
                            },
                        )
                        .await?
                        {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                        }
                    } else {
                        if !Repository::<Postgresql<UserRegistrationToken>>::delete(
                            &postgresql_client_database_2,
                            UserRegistrationTokenBy {
                                user__email: incoming.user__email,
                                user_device__id: incoming.user_device__id,
                            },
                        )
                        .await?
                        {
                            return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                        }
                    }
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken__WrongValue));
                }
            }
            let user__password = incoming.user__password.to_string();
            let user__password_hash = crate::result_return_runtime!(
                Spawner::<TokioBlockingTask>::spawn_processed(
                    move || -> _ {
                        return Encoder::<User_Password>::encode(user__password.as_str());
                    },
                )
                .await
            )?;
            let postgresql_client_database_1 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            let user__id = Repository::<Postgresql<User>>::get_id(&postgresql_client_database_1).await?;
            let user_access_token__obfuscation_value = Generator::<UserAccessToken_ObfuscationValue>::generate();
            let user_access_token__expires_at = Generator::<UserAccessToken_ExpiresAt>::generate(now)?;
            let user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?;
            let postgresql_client_database_3 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_3.get().await);
            let mut postgresql_client_database_2 = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_client_database_2,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let is_created = match Repository::<Postgresql<UserAccessRefreshToken>>::create(
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
            if !is_created {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
            }
            let is_deleted = match Repository::<Postgresql<UserRegistrationToken>>::delete(
                transaction.get_client(),
                UserRegistrationTokenBy {
                    user__email: incoming.user__email,
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
            let is_created = match Repository::<Postgresql<User>>::create_2(
                &postgresql_client_database_1,
                UserInsert2 {
                    user__id,
                    user__email: incoming.user__email,
                    user__nickname: incoming.user__nickname,
                    user__password_hash: user__password_hash.as_str(),
                    user__created_at: now,
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
            if let Result::Err(aggregate_error) = Resolver_::<Transaction<'_>>::commit(transaction).await {
                if !Repository::<Postgresql<User>>::delete(
                    &postgresql_client_database_1,
                    UserBy3 {
                        user__id,
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::ParallelExecution));
                }
                return Result::Err(aggregate_error);
            }
            let user_access_token_signed = Encoder::<UserAccessToken>::encode(
                &inner.environment_configuration.subject.encryption.private_key,
                user__id,
                incoming.user_device__id,
                user_access_token__obfuscation_value,
                user_access_token__expires_at,
            )?;
            let user_access_refresh_token_signed = Encoder::<UserAccessRefreshToken>::encode(
                &inner.environment_configuration.subject.encryption.private_key,
                user__id,
                incoming.user_device__id,
                user_access_token__obfuscation_value,
                user_access_refresh_token__obfuscation_value,
                user_access_refresh_token__expires_at,
                now,
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
                                user__id,
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
