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
                User_Id,
            },
            user_access_refresh_token::{
                UserAccessRefreshToken,
                UserAccessRefreshToken_ExpiresAt,
                UserAccessRefreshToken_ObfuscationValue,
                UserAccessRefreshToken_UpdatedAt,
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
        data::capture::Capture,
        functionality::{
            repository::postgresql::{
                user::By3,
                user_access_refresh_token::{
                    By2,
                    Insert1 as UserAccessRefreshTokenInsert1,
                    Update1,
                },
                user_authorization_token::{
                    By1,
                    Update4,
                },
                user_device::Insert1 as UserDeviceInsert1,
                PostgresqlRepository,
            },
            service::{
                resolver::{
                    Expiration,
                    Resolver,
                },
                spawner::{
                    TokioNonBlockingTask,
                    Spawner,
                },
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::user_authorization::authorize_by_last_step::{
    Incoming,
    Outcoming,
    Precedent,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    OptionConverter,
    ResultConverter,
};
use std::{
    borrow::Cow,
    future::Future,
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub struct UserAuthorization_AuthorizeByLastStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_AuthorizeByLastStep> {
    type Incoming = Incoming;
    type Outcoming = Outcoming;
    type Precedent = Precedent;
    fn process<'a, T>(
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return async move {
            if !Validator::<User_Id>::is_valid(incoming.user__id) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<UserAuthorizationToken_Value>::is_valid(incoming.user_authorization_token__value.as_str())? {
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
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let user_authorization_token = PostgresqlRepository::<UserAuthorizationToken>::find_2(
                database_2_postgresql_connection,
                By1 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            let mut user_authorization_token_ = match user_authorization_token {
                Option::Some(user_authorization_token__) => user_authorization_token__,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(user_authorization_token_.expires_at) {
                PostgresqlRepository::<UserAuthorizationToken<'_>>::delete_1(
                    database_2_postgresql_connection,
                    By1 {
                        user__id: incoming.user__id,
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_AlreadyExpired));
            }
            if user_authorization_token_.value != incoming.user_authorization_token__value {
                user_authorization_token_.wrong_enter_tries_quantity =
                    user_authorization_token_.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                if user_authorization_token_.wrong_enter_tries_quantity < UserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                    PostgresqlRepository::<UserAuthorizationToken>::update_4(
                        database_2_postgresql_connection,
                        Update4 {
                            user_authorization_token__wrong_enter_tries_quantity: user_authorization_token_.wrong_enter_tries_quantity,
                        },
                        By1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    PostgresqlRepository::<UserAuthorizationToken<'_>>::delete_1(
                        database_2_postgresql_connection,
                        By1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserAuthorizationToken_WrongValue {
                            user_authorization_token__wrong_enter_tries_quantity: user_authorization_token_.wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
            if !PostgresqlRepository::<User<'_>>::is_exist_3(
                database_1_postgresql_connection,
                By3 {
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
                Generator::<UserAccessToken_ExpiresAt>::generate()?,
            );
            let user_access_token__id = user_access_token.id.as_str();
            let user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate()?;
            let user_access_refresh_token__updated_at = Generator::<UserAccessRefreshToken_UpdatedAt>::generate();
            // TODO  TRANZACTION
            let user_access_refresh_token = match PostgresqlRepository::<UserAccessRefreshToken<'_>>::find_1(
                database_2_postgresql_connection,
                By2 {
                    user__id: incoming.user__id,
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(mut user_access_refresh_token_) => {
                    user_access_refresh_token_.user_access_token__id = Cow::Borrowed(user_access_token__id);
                    user_access_refresh_token_.obfuscation_value = user_access_refresh_token__obfuscation_value;
                    user_access_refresh_token_.expires_at = user_access_refresh_token__expires_at;
                    user_access_refresh_token_.updated_at = user_access_refresh_token__updated_at;
                    PostgresqlRepository::<UserAccessRefreshToken>::update_1(
                        database_2_postgresql_connection,
                        Update1 {
                            user_access_token__id: user_access_refresh_token_.user_access_token__id.as_ref(),
                            user_access_refresh_token__obfuscation_value: user_access_refresh_token_.obfuscation_value.as_str(),
                            user_access_refresh_token__expires_at: user_access_refresh_token_.expires_at,
                            user_access_refresh_token__updated_at: user_access_refresh_token_.updated_at,
                        },
                        By2 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                        },
                    )
                    .await?;
                    user_access_refresh_token_
                }
                Option::None => {
                    let user_access_refresh_token_ = PostgresqlRepository::<UserAccessRefreshToken<'_>>::create_1(
                        database_2_postgresql_connection,
                        UserAccessRefreshTokenInsert1 {
                            user__id: incoming.user__id,
                            user_device__id: incoming.user_device__id.as_str(),
                            user_access_token__id,
                            user_access_refresh_token__obfuscation_value,
                            user_access_refresh_token__expires_at,
                            user_access_refresh_token__updated_at,
                        },
                    )
                    .await?;
                    user_access_refresh_token_
                }
            };
            // TODO  TRANZACTION
            let user_access_token_encoded = Encoder::<UserAccessToken<'_>>::encode(
                inner.environment_configuration,
                &user_access_token,
            )?;
            let user_access_refresh_token_encoded = Encoder::<UserAccessRefreshToken<'_>>::encode(
                inner.environment_configuration,
                &user_access_refresh_token,
            )?;
            let database_1_postgresql_connection_pool = inner.database_1_postgresql_connection_pool.clone();
            let database_2_postgresql_connection_pool = inner.database_2_postgresql_connection_pool.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let user_device = PostgresqlRepository::<UserDevice>::create_1(
                        &*database_1_postgresql_connection_pool.get().await.into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        UserDeviceInsert1 {
                            user_device__id: incoming.user_device__id,
                            user__id: incoming.user__id,
                        },
                    )
                    .await?;
                    PostgresqlRepository::<UserAuthorizationToken<'_>>::delete_1(
                        &*database_2_postgresql_connection_pool.get().await.into_runtime(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
                        By1 {
                            user__id: user_device.user__id,
                            user_device__id: user_device.id.as_str(),
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
