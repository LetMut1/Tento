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
                application_user::By3,
                application_user_access_refresh_token::{
                    By2,
                    Insert1 as ApplicationUserAccessRefreshTokenInsert1,
                    Update1,
                },
                application_user_authorization_token::{
                    By1,
                    Update4,
                },
                application_user_device::Insert1 as ApplicationUserDeviceInsert1,
                PostgresqlRepository,
            },
            service::{
                resolver::{
                    expiration::Expiration,
                    Resolver,
                },
                spawner::{
                    tokio_non_blocking_task::TokioNonBlockingTask,
                    Spawner,
                },
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::{
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
pub struct ApplicationUser__Authorization___AuthorizeByLastStep;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___AuthorizeByLastStep> {
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
            if !Validator::<User_Id>::is_valid(incoming.application_user__id) {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<UserAuthorizationToken_Value>::is_valid(incoming.application_user_authorization_token__value.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<UserDevice_Id>::is_valid(incoming.application_user_device__id.as_str()) {
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
            let application_user_authorization_token = PostgresqlRepository::<UserAuthorizationToken>::find_2(
                database_2_postgresql_connection,
                By1 {
                    application_user__id: incoming.application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?;
            let mut application_user_authorization_token_ = match application_user_authorization_token {
                Option::Some(application_user_authorization_token__) => application_user_authorization_token__,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(application_user_authorization_token_.expires_at) {
                PostgresqlRepository::<UserAuthorizationToken<'_>>::delete_1(
                    database_2_postgresql_connection,
                    By1 {
                        application_user__id: incoming.application_user__id,
                        application_user_device__id: incoming.application_user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_AlreadyExpired));
            }
            if application_user_authorization_token_.value != incoming.application_user_authorization_token__value {
                application_user_authorization_token_.wrong_enter_tries_quantity =
                    application_user_authorization_token_.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                if application_user_authorization_token_.wrong_enter_tries_quantity < UserAuthorizationToken_WrongEnterTriesQuantity::LIMIT {
                    PostgresqlRepository::<UserAuthorizationToken>::update_4(
                        database_2_postgresql_connection,
                        Update4 {
                            application_user_authorization_token__wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                        },
                        By1 {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    PostgresqlRepository::<UserAuthorizationToken<'_>>::delete_1(
                        database_2_postgresql_connection,
                        By1 {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(
                    UnifiedReport::precedent(
                        Precedent::UserAuthorizationToken_WrongValue {
                            application_user_authorization_token__wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
            if !PostgresqlRepository::<User<'_>>::is_exist_3(
                database_1_postgresql_connection,
                By3 {
                    application_user__id: incoming.application_user__id,
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_NotFound));
            }
            let application_user_access_token = UserAccessToken::new(
                Generator::<UserAccessToken_Id>::generate(),
                incoming.application_user__id,
                Cow::Borrowed(incoming.application_user_device__id.as_str()),
                Generator::<UserAccessToken_ExpiresAt>::generate()?,
            );
            let application_user_access_token__id = application_user_access_token.id.as_str();
            let application_user_access_refresh_token__obfuscation_value = Generator::<UserAccessRefreshToken_ObfuscationValue>::generate();
            let application_user_access_refresh_token__expires_at = Generator::<UserAccessRefreshToken_ExpiresAt>::generate()?;
            let application_user_access_refresh_token__updated_at = Generator::<UserAccessRefreshToken_UpdatedAt>::generate();
            // TODO  TRANZACTION
            let application_user_access_refresh_token = match PostgresqlRepository::<UserAccessRefreshToken<'_>>::find_1(
                database_2_postgresql_connection,
                By2 {
                    application_user__id: incoming.application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(mut application_user_access_refresh_token_) => {
                    application_user_access_refresh_token_.application_user_access_token__id = Cow::Borrowed(application_user_access_token__id);
                    application_user_access_refresh_token_.obfuscation_value = application_user_access_refresh_token__obfuscation_value;
                    application_user_access_refresh_token_.expires_at = application_user_access_refresh_token__expires_at;
                    application_user_access_refresh_token_.updated_at = application_user_access_refresh_token__updated_at;
                    PostgresqlRepository::<UserAccessRefreshToken>::update_1(
                        database_2_postgresql_connection,
                        Update1 {
                            application_user_access_token__id: application_user_access_refresh_token_.application_user_access_token__id.as_ref(),
                            application_user_access_refresh_token__obfuscation_value: application_user_access_refresh_token_.obfuscation_value.as_str(),
                            application_user_access_refresh_token__expires_at: application_user_access_refresh_token_.expires_at,
                            application_user_access_refresh_token__updated_at: application_user_access_refresh_token_.updated_at,
                        },
                        By2 {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                    application_user_access_refresh_token_
                }
                Option::None => {
                    let application_user_access_refresh_token_ = PostgresqlRepository::<UserAccessRefreshToken<'_>>::create_1(
                        database_2_postgresql_connection,
                        ApplicationUserAccessRefreshTokenInsert1 {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                            application_user_access_token__id,
                            application_user_access_refresh_token__obfuscation_value,
                            application_user_access_refresh_token__expires_at,
                            application_user_access_refresh_token__updated_at,
                        },
                    )
                    .await?;
                    application_user_access_refresh_token_
                }
            };
            // TODO  TRANZACTION
            let application_user_access_token_encoded = Encoder::<UserAccessToken<'_>>::encode(
                inner.environment_configuration,
                &application_user_access_token,
            )?;
            let application_user_access_refresh_token_encoded = Encoder::<UserAccessRefreshToken<'_>>::encode(
                inner.environment_configuration,
                &application_user_access_refresh_token,
            )?;
            let database_1_postgresql_connection_pool = inner.database_1_postgresql_connection_pool.clone();
            let database_2_postgresql_connection_pool = inner.database_2_postgresql_connection_pool.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.into_runtime(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                    let application_user_device = PostgresqlRepository::<UserDevice>::create_1(
                        &*database_1_postgresql_pooled_connection,
                        ApplicationUserDeviceInsert1 {
                            application_user_device__id: incoming.application_user_device__id,
                            application_user__id: incoming.application_user__id,
                        },
                    )
                    .await?;
                    let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.into_runtime(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                    PostgresqlRepository::<UserAuthorizationToken<'_>>::delete_1(
                        &*database_2_postgresql_pooled_connection,
                        By1 {
                            application_user__id: application_user_device.application_user__id,
                            application_user_device__id: application_user_device.id.as_str(),
                        },
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                application_user_access_token_encoded,
                application_user_access_refresh_token_encoded,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
