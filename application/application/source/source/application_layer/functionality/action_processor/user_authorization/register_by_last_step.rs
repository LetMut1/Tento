use crate::{
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
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            capture::Capture,
        },
        functionality::{
            repository::{
                postgresql::{
                    Postgresql,
                    UserBy1,
                    UserBy2,
                    UserBy3,
                    UserDeviceInsert1,
                    UserRegistrationTokenBy1,
                    Resolver as Resolver_,
                    Transaction,
                    IsolationLevel,
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
};
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::register_by_last_step::{
        Incoming,
        Outcoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::{
    borrow::Cow,
    future::Future
};
pub struct UserAuthorization_RegisterByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RegisterByLastStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(
        inner: &'a Inner<'_>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            if !Validator::<User_Password>::is_valid(
                incoming.user__password.as_str(),
                incoming.user__email.as_str(),
                incoming.user__nickname.as_str(),
            ) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<User_Nickname>::is_valid(incoming.user__nickname.as_str()) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<User_Email>::is_valid(incoming.user__email.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<UserRegistrationToken_Value>::is_valid(incoming.user_registration_token__value.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            {
                let postgresql_database_1_client = inner.postgresql_connection_pool_database_1.get().await.into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                if Repository::<Postgresql<User<'_>>>::is_exist_1(
                    &postgresql_database_1_client,
                    UserBy1 {
                        user__nickname: incoming.user__nickname.as_str(),
                    },
                )
                .await?
                {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User_NicknameAlreadyExist));
                }
                if Repository::<Postgresql<User<'_>>>::is_exist_2(
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
            let now = Resolver::<UnixTime>::get_now();
            {
                let postgresql_database_2_client = inner.postgresql_connection_pool_database_2.get().await.into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
                let mut user_registration_token = match Repository::<Postgresql<UserRegistrationToken<'_>>>::find_2(
                    &postgresql_database_2_client,
                    UserRegistrationTokenBy1 {
                        user__email: incoming.user__email.as_str(),
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?
                {
                    Option::Some(user_registration_token_) => user_registration_token_,
                    Option::None => {
                        return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_NotFound));
                    }
                };
                if user_registration_token.expires_at <= now {
                    Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                        &postgresql_database_2_client,
                        UserRegistrationTokenBy1 {
                            user__email: incoming.user__email.as_str(),
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_AlreadyExpired));
                }
                if !user_registration_token.is_approved {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_IsNotApproved));
                }
                if user_registration_token.value != incoming.user_registration_token__value {
                    if user_registration_token.wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                        user_registration_token.wrong_enter_tries_quantity += 1;
                    }
                    if user_registration_token.wrong_enter_tries_quantity < UserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                        Repository::<Postgresql<UserRegistrationToken<'_>>>::update_4(
                            &postgresql_database_2_client,
                            UserRegistrationTokenBy1 {
                                user__email: incoming.user__email.as_str(),
                                user_device__id: incoming.user_device__id.as_str(),
                            },
                        )
                        .await?;
                    } else {
                        Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                            &postgresql_database_2_client,
                            UserRegistrationTokenBy1 {
                                user__email: incoming.user__email.as_str(),
                                user_device__id: incoming.user_device__id.as_str(),
                            },
                        )
                        .await?;
                    }
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_WrongValue));
                }
            }
            let user__password_hash = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<User_Password>::encode(incoming.user__password.as_str());
                },
            )
            .await
            .into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )??;
            let postgresql_database_1_client = inner.postgresql_connection_pool_database_1.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let user__id = Repository::<Postgresql<User<'_>>>::get_user_id(&postgresql_database_1_client).await?;
            let user = User::new(
                user__id,
                incoming.user__email,
                Cow::Owned(incoming.user__nickname),
                user__password_hash,
                now,
            );
            let user_access_token = UserAccessToken::new(
                Generator::<UserAccessToken_Id>::generate(),
                user.id,
                incoming.user_device__id.as_str(),
                Generator::<UserAccessToken_ExpiresAt>::generate(now)?,
            );
            let user_access_refresh_token = UserAccessRefreshToken::new(
                user.id,
                incoming.user_device__id.as_str(),
                Cow::Borrowed(user_access_token.id.as_str()),
                Generator::<UserAccessRefreshToken_ObfuscationValue>::generate(),
                Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?,
                now,
            );
            let mut postgresql_database_2_client = inner.postgresql_connection_pool_database_2.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_2_client,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            if let Result::Err(aggregate_error) = Repository::<Postgresql<UserAccessRefreshToken<'_>>>::create_1(
                transaction.get_client(),
                &user_access_refresh_token,
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            };
            if let Result::Err(aggregate_error) = Repository::<Postgresql<UserRegistrationToken<'_>>>::delete_2(
                transaction.get_client(),
                UserRegistrationTokenBy1 {
                    user__email: user.email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Repository::<Postgresql<User<'_>>>::create_2(
                &postgresql_database_1_client,
                &user,
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            if let Result::Err(aggregate_error) = Resolver_::<Transaction<'_>>::commit(transaction).await {
                Repository::<Postgresql<User<'_>>>::delete_1(
                    &postgresql_database_1_client,
                    UserBy3 {
                        user__id: user.id,
                    },
                )
                .await?;
                return Result::Err(aggregate_error);
            }
            let user_access_token_encoded = Encoder::<UserAccessToken<'_>>::encode(
                &inner.environment_configuration.encryption.private_key,
                &user_access_token,
            )?;
            let user_access_refresh_token_encoded = Encoder::<UserAccessRefreshToken<'_>>::encode(
                &inner.environment_configuration.encryption.private_key,
                &user_access_refresh_token,
            )?;
            let postgresql_connection_pool_database_1 = inner.postgresql_connection_pool_database_1.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let user_device = Repository::<Postgresql<UserDevice>>::create_1(
                        &postgresql_connection_pool_database_1.get().await.into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        UserDeviceInsert1 {
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
