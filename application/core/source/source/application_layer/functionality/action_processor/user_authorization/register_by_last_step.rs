use {
    crate::{
        application_layer::functionality::action_processor::{
            ActionProcessor,
            ActionProcessor_,
            Inner,
        },
        domain_layer::{
            data::entity::{
                user::{
                    User,
                    User_Email,
                    User_Nickname,
                    User_Password,
                },
                user_access_refresh_token::{
                    UserAccessRefreshToken,
                    UserAccessRefreshToken_ExpiresAt,
                    UserAccessRefreshToken_ObfuscationValue,
                },
                user_access_token::{
                    UserAccessToken,
                    UserAccessToken_ExpiresAt,
                    UserAccessToken_Id,
                },
                user_device::{
                    UserDevice,
                    UserDevice_Id,
                },
                user_registration_token::{
                    UserRegistrationToken,
                    UserRegistrationToken_Value,
                    UserRegistrationToken_WrongEnterTriesQuantity,
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
                    postgresql::{
                        IsolationLevel,
                        Postgresql,
                        Resolver as Resolver_,
                        Transaction,
                        UserBy1,
                        UserBy2,
                        UserBy3,
                        UserDeviceInsert,
                        UserRegistrationTokenBy,
                        UserAccessRefreshTokenInsert,
                    },
                    Repository,
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
        },
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::user_authorization::register_by_last_step::{
            Incoming,
            Outcoming,
            Precedent,
        },
        unified_report::UnifiedReport,
    },
    std::future::Future
};
pub struct UserAuthorization_RegisterByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RegisterByLastStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Password>::is_valid(
                incoming.user__password.as_str(),
                incoming.user__email.as_str(),
                incoming.user__nickname.as_str(),
            ) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Nickname>::is_valid(incoming.user__nickname.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<User_Email>::is_valid(incoming.user__email.as_str())? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserRegistrationToken_Value>::is_valid(incoming.user_registration_token__value.as_str())? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            {
                let postgresql_database_1_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
                if Repository::<Postgresql<User>>::is_exist_1(
                    &postgresql_database_1_client,
                    UserBy1 {
                        user__nickname: incoming.user__nickname.as_str(),
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User_NicknameAlreadyExist));
                }
                if Repository::<Postgresql<User>>::is_exist_2(
                    &postgresql_database_1_client,
                    UserBy2 {
                        user__email: incoming.user__email.as_str(),
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User_EmailAlreadyExist));
                }
            }
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            {
                let postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
                let (
                    user_registration_token__value,
                    mut user_registration_token__wrong_enter_tries_quantity,
                    user_registration_token__is_approved,
                    user_registration_token__expires_at,
                ) = match Repository::<Postgresql<UserRegistrationToken<'_>>>::find_2(
                    &postgresql_database_2_client,
                    UserRegistrationTokenBy {
                        user__email: incoming.user__email.as_str(),
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?
                {
                    Option::Some(values) => values,
                    Option::None => {
                        return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_NotFound));
                    }
                };
                if user_registration_token__expires_at <= now {
                    Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                        &postgresql_database_2_client,
                        UserRegistrationTokenBy {
                            user__email: incoming.user__email.as_str(),
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_AlreadyExpired));
                }
                if !user_registration_token__is_approved {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_IsNotApproved));
                }
                if user_registration_token__value != incoming.user_registration_token__value {
                    if user_registration_token__wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                        user_registration_token__wrong_enter_tries_quantity += 1;
                    }
                    if user_registration_token__wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                        Repository::<Postgresql<UserRegistrationToken<'_>>>::update_4(
                            &postgresql_database_2_client,
                            UserRegistrationTokenBy {
                                user__email: incoming.user__email.as_str(),
                                user_device__id: incoming.user_device__id.as_str(),
                            },
                        )
                        .await?;
                    } else {
                        Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                            &postgresql_database_2_client,
                            UserRegistrationTokenBy {
                                user__email: incoming.user__email.as_str(),
                                user_device__id: incoming.user_device__id.as_str(),
                            },
                        )
                        .await?;
                    }
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_WrongValue));
                }
            }
            let user__password_hash = crate::result_return_runtime!(
                Spawner::<TokioBlockingTask>::spawn_processed(
                    move || -> _ {
                        return Encoder::<User_Password>::encode(incoming.user__password.as_str());
                    },
                )
                .await
            )?;
            let postgresql_database_1_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_1.get().await);
            let user__id = Repository::<Postgresql<User>>::get_id(&postgresql_database_1_client).await?;
            let user = User::new(
                user__id,
                incoming.user__email,
                incoming.user__nickname,
                user__password_hash,
                now,
            );
            let user_access_token__id = Generator::<UserAccessToken_Id>::generate();
            let user_access_token__expires_at = Generator::<UserAccessToken_ExpiresAt>::generate(now)?;
            let user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?;
            let user_access_refresh_token__updated_at = now;
            let mut postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_2_client,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            if let Result::Err(aggregate_error) = Repository::<Postgresql<UserAccessRefreshToken>>::create_1(
                transaction.get_client(),
                &UserAccessRefreshTokenInsert {
                    user__id: user.id,
                    user_device__id: incoming.user_device__id.as_str(),
                    user_access_token__id: user_access_token__id.as_str(),
                    user_access_refresh_token__obfuscation_value: user_access_refresh_token__obfuscation_value.as_str(),
                    user_access_refresh_token__expires_at,
                    user_access_refresh_token__updated_at,
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            };
            if let Result::Err(aggregate_error) = Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                transaction.get_client(),
                UserRegistrationTokenBy {
                    user__email: user.email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Repository::<Postgresql<User>>::create_2(
                &postgresql_database_1_client,
                &user,
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Resolver_::<Transaction<'_>>::commit(transaction).await {
                Repository::<Postgresql<User>>::delete_1(
                    &postgresql_database_1_client,
                    UserBy3 {
                        user__id: user.id,
                    },
                )
                .await?;
                return Result::Err(aggregate_error);
            }
            let user_access_token_encoded = Encoder::<UserAccessToken>::encode(
                &inner.environment_configuration.subject.encryption.private_key,
                user_access_token__id.as_str(),
                user.id,
                incoming.user_device__id.as_str(),
                user_access_token__expires_at,
            )?;
            let user_access_refresh_token_encoded = Encoder::<UserAccessRefreshToken>::encode(
                &inner.environment_configuration.subject.encryption.private_key,
                user__id,
                incoming.user_device__id.as_str(),
                user_access_token__id.as_str(),
                user_access_refresh_token__obfuscation_value.as_str(),
                user_access_refresh_token__expires_at,
                user_access_refresh_token__updated_at,
            )?;
            let postgresql_connection_pool_database_1 = inner.postgresql_connection_pool_database_1.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let user_device = Repository::<Postgresql<UserDevice>>::create_1(
                        &crate::result_return_runtime!(postgresql_connection_pool_database_1.get().await),
                        UserDeviceInsert {
                            user_device__id: incoming.user_device__id,
                            user__id,
                        },
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                user_access_token_encoded,
                user_access_refresh_token_encoded,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
