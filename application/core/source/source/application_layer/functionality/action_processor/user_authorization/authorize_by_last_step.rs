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
                    UserAccessToken_Id,
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
                    postgresql::{
                        IsolationLevel,
                        Postgresql,
                        Resolver as Resolver_,
                        Transaction,
                        UserAccessRefreshTokenBy2,
                        UserAccessRefreshTokenUpdate1,
                        UserAuthorizationTokenBy1,
                        UserBy3,
                        UserDeviceInsert1,
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
        borrow::Cow,
        future::Future,
    },
};
pub struct UserAuthorization_AuthorizeByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_AuthorizeByLastStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a>(inner: &'a Inner<'_>, incoming: Self::Incoming) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send {
        return async move {
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserAuthorizationToken_Value>::is_valid(incoming.user_authorization_token__value.as_str())? {
                return Result::Err(crate::new_invalid_argument!());
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.user_device__id.as_str()) {
                return Result::Err(crate::new_invalid_argument!());
            }
            let mut postgresql_database_2_client = crate::result_return_runtime!(inner.postgresql_connection_pool_database_2.get().await);
            let (
                user_authorization_token__value,
                mut user_authorization_token__wrong_enter_tries_quantity,
                user_authorization_token__expires_at,
            ) = match Repository::<Postgresql<UserAuthorizationToken<'_>>>::find_2(
                &postgresql_database_2_client,
                UserAuthorizationTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(values) => values,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_NotFound));
                }
            };
            let now = Resolver::<UnixTime>::get_now_in_seconds();
            if user_authorization_token__expires_at <= now {
                Repository::<Postgresql<UserAuthorizationToken<'_>>>::delete_1(
                    &postgresql_database_2_client,
                    UserAuthorizationTokenBy1 {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_AlreadyExpired));
            }
            if user_authorization_token__value != incoming.user_authorization_token__value {
                if user_authorization_token__wrong_enter_tries_quantity < UserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                    user_authorization_token__wrong_enter_tries_quantity += 1;
                }
                if user_authorization_token__wrong_enter_tries_quantity < UserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                    Repository::<Postgresql<UserAuthorizationToken<'_>>>::update_4(
                        &postgresql_database_2_client,
                        UserAuthorizationTokenBy1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    Repository::<Postgresql<UserAuthorizationToken<'_>>>::delete_1(
                        &postgresql_database_2_client,
                        UserAuthorizationTokenBy1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserAuthorizationToken_WrongValue {
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
                return Result::Ok(UnifiedReport::precedent(Precedent::User_NotFound));
            }
            let user_access_token = UserAccessToken::new(
                Generator::<UserAccessToken_Id>::generate(),
                incoming.user__id,
                incoming.user_device__id.as_str(),
                Generator::<UserAccessToken_ExpiresAt>::generate(now)?,
            );
            let user_access_token__id = user_access_token.id.as_str();
            let user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate(now)?;
            let user_access_refresh_token__updated_at = now;
            let user_access_refresh_token = Repository::<Postgresql<UserAccessRefreshToken<'_>>>::find_1(
                &postgresql_database_2_client,
                UserAccessRefreshTokenBy2 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            let transaction = Resolver_::<Transaction<'_>>::start(
                &mut postgresql_database_2_client,
                IsolationLevel::ReadCommitted,
            )
            .await?;
            let user_access_refresh_token_ = match user_access_refresh_token {
                Option::Some(mut user_access_refresh_token__) => {
                    user_access_refresh_token__.user_access_token__id = Cow::Borrowed(user_access_token__id);
                    user_access_refresh_token__.obfuscation_value = user_access_refresh_token__obfuscation_value;
                    user_access_refresh_token__.expires_at = user_access_refresh_token__expires_at;
                    user_access_refresh_token__.updated_at = user_access_refresh_token__updated_at;
                    if let Result::Err(aggregate_error) = Repository::<Postgresql<UserAccessRefreshToken<'_>>>::update_1(
                        transaction.get_client(),
                        UserAccessRefreshTokenUpdate1 {
                            user_access_token__id: user_access_refresh_token__.user_access_token__id.as_ref(),
                            user_access_refresh_token__obfuscation_value: user_access_refresh_token__.obfuscation_value.as_str(),
                            user_access_refresh_token__expires_at: user_access_refresh_token__.expires_at,
                            user_access_refresh_token__updated_at: user_access_refresh_token__.updated_at,
                        },
                        UserAccessRefreshTokenBy2 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await
                    {
                        Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                        return Result::Err(aggregate_error);
                    }
                    user_access_refresh_token__
                }
                Option::None => {
                    let user_access_refresh_token__ = UserAccessRefreshToken::new(
                        incoming.user__id,
                        incoming.user_device__id.as_str(),
                        Cow::Borrowed(user_access_token__id),
                        user_access_refresh_token__obfuscation_value,
                        user_access_refresh_token__expires_at,
                        user_access_refresh_token__updated_at,
                    );
                    if let Result::Err(aggregate_error) = Repository::<Postgresql<UserAccessRefreshToken<'_>>>::create_1(
                        transaction.get_client(),
                        &user_access_refresh_token__,
                    )
                    .await
                    {
                        Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                        return Result::Err(aggregate_error);
                    };
                    user_access_refresh_token__
                }
            };
            if let Result::Err(aggregate_error) = Repository::<Postgresql<UserAuthorizationToken<'_>>>::delete_1(
                transaction.get_client(),
                UserAuthorizationTokenBy1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await
            {
                Resolver_::<Transaction<'_>>::rollback(transaction).await?;
                return Result::Err(aggregate_error);
            }
            Resolver_::<Transaction<'_>>::commit(transaction).await?;
            let user_access_token_encoded = Encoder::<UserAccessToken<'_>>::encode(
                &inner.environment_configuration.subject.encryption.private_key,
                &user_access_token,
            )?;
            let user_access_refresh_token_encoded = Encoder::<UserAccessRefreshToken<'_>>::encode(
                &inner.environment_configuration.subject.encryption.private_key,
                user_access_refresh_token_.user__id,
                user_access_refresh_token_.user_device__id,
                user_access_refresh_token_.user_access_token__id.as_ref(),
                user_access_refresh_token_.obfuscation_value.as_str(),
                user_access_refresh_token_.expires_at,
                user_access_refresh_token_.updated_at,
            )?;
            let postgresql_connection_pool_database_1 = inner.postgresql_connection_pool_database_1.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let user_device = Repository::<Postgresql<UserDevice>>::create_1(
                        &crate::result_return_runtime!(postgresql_connection_pool_database_1.get().await),
                        UserDeviceInsert1 {
                            user_device__id: incoming.user_device__id,
                            user__id: incoming.user__id,
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
