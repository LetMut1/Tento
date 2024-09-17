use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            application_user::{
                ApplicationUser,
                ApplicationUser_Email,
                ApplicationUser_Nickname,
                ApplicationUser_Password,
            },
            application_user_access_refresh_token::{
                ApplicationUserAccessRefreshToken,
                ApplicationUserAccessRefreshToken_ExpiresAt,
                ApplicationUserAccessRefreshToken_ObfuscationValue,
                ApplicationUserAccessRefreshToken_UpdatedAt,
            },
            application_user_access_token::{
                ApplicationUserAccessToken,
                ApplicationUserAccessToken_ExpiresAt,
                ApplicationUserAccessToken_Id,
            },
            application_user_device::{
                ApplicationUserDevice,
                ApplicationUserDevice_Id,
            },
            application_user_registration_token::{
                ApplicationUserRegistrationToken,
                ApplicationUserRegistrationToken_Value,
                ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
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
                application_user::{
                    By1,
                    By2,
                    Insert1 as ApplicationUserInsert1,
                },
                application_user_access_refresh_token::Insert1 as ApplicationUserAccessRefreshTokenInsert1,
                application_user_device::Insert1,
                application_user_registration_token::{
                    By1 as By1_,
                    Update4,
                },
                PostgresqlRepository,
            },
            service::{
                resolver::{
                    expiration::Expiration,
                    Resolver,
                },
                spawner::{
                    tokio_blocking_task::TokioBlockingTask,
                    tokio_non_blocking_task::TokioNonBlockingTask,
                    Spawner,
                },
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::{
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
pub struct ApplicationUser__Authorization___RegisterByLastStep;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___RegisterByLastStep> {
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
            if !Validator::<ApplicationUser_Password>::is_valid(
                incoming.application_user__password.as_str(),
                incoming.application_user__email.as_str(),
                incoming.application_user__nickname.as_str(),
            ) {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUser_Nickname>::is_valid(incoming.application_user__nickname.as_str()) {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUser_Email>::is_valid(incoming.application_user__email.as_str())? {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUserRegistrationToken_Value>::is_valid(incoming.application_user_registration_token__value.as_str())? {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming.application_user_device__id.as_str()) {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
            if PostgresqlRepository::<ApplicationUser<'_>>::is_exist_1(
                database_1_postgresql_connection,
                By1 {
                    application_user__nickname: incoming.application_user__nickname.as_str(),
                },
            )
            .await?
            {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_NicknameAlreadyExist));
            }
            if PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
                database_1_postgresql_connection,
                By2 {
                    application_user__email: incoming.application_user__email.as_str(),
                },
            )
            .await?
            {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_EmailAlreadyExist));
            }
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken>::find_2(
                database_2_postgresql_connection,
                By1_ {
                    application_user__email: incoming.application_user__email.as_str(),
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Some(application_user_registration_token_) => application_user_registration_token_,
                None => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(application_user_registration_token.expires_at) {
                PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1_ {
                        application_user__email: incoming.application_user__email.as_str(),
                        application_user_device__id: incoming.application_user_device__id.as_str(),
                    },
                )
                .await?;
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyExpired));
            }
            if !application_user_registration_token.is_approved {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_IsNotApproved));
            }
            if application_user_registration_token.value != incoming.application_user_registration_token__value {
                application_user_registration_token.wrong_enter_tries_quantity =
                    application_user_registration_token.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                if application_user_registration_token.wrong_enter_tries_quantity < ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT {
                    PostgresqlRepository::<ApplicationUserRegistrationToken>::update_4(
                        database_2_postgresql_connection,
                        Update4 {
                            application_user_registration_token__wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                        },
                        By1_ {
                            application_user__email: incoming.application_user__email.as_str(),
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                        database_2_postgresql_connection,
                        By1_ {
                            application_user__email: incoming.application_user__email.as_str(),
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_WrongValue));
            }
            let application_user__password_hash___join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<ApplicationUser_Password>::encode(incoming.application_user__password.as_str());
                },
            );
            let application_user = PostgresqlRepository::<ApplicationUser<'_>>::create_1(
                database_1_postgresql_connection,
                ApplicationUserInsert1 {
                    application_user__email: incoming.application_user__email,
                    application_user__nickname: incoming.application_user__nickname,
                    application_user__password_hash: application_user__password_hash___join_handle.await.into_runtime(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )??,
                },
            )
            .await?;
            let application_user_access_token = ApplicationUserAccessToken::new(
                Generator::<ApplicationUserAccessToken_Id>::generate(),
                application_user.id,
                Cow::Borrowed(incoming.application_user_device__id.as_str()),
                Generator::<ApplicationUserAccessToken_ExpiresAt>::generate()?,
            );
            // TODO  TRANZACTION посмотреть, необходимо ли здесь сделать транзакцию
            let application_user_access_refresh_token = PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::create_1(
                database_2_postgresql_connection,
                ApplicationUserAccessRefreshTokenInsert1 {
                    application_user__id: application_user.id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                    application_user_access_token__id: application_user_access_token.id.as_str(),
                    application_user_access_refresh_token__obfuscation_value: Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate(),
                    application_user_access_refresh_token__expires_at: Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate()?,
                    application_user_access_refresh_token__updated_at: Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate(),
                },
            )
            .await?;
            let application_user_access_token_encoded = Encoder::<ApplicationUserAccessToken<'_>>::encode(
                inner.environment_configuration,
                &application_user_access_token,
            )?;
            let application_user_access_refresh_token_encoded = Encoder::<ApplicationUserAccessRefreshToken<'_>>::encode(
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
                    let application_user_device = PostgresqlRepository::<ApplicationUserDevice>::create_1(
                        &*database_1_postgresql_pooled_connection,
                        Insert1 {
                            application_user_device__id: incoming.application_user_device__id,
                            application_user__id: application_user.id,
                        },
                    )
                    .await?;
                    let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.into_runtime(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                    PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete_2(
                        &*database_2_postgresql_pooled_connection,
                        By1_ {
                            application_user__email: application_user.email.as_str(),
                            application_user_device__id: application_user_device.id.as_str(),
                        },
                    )
                    .await?;
                    return Ok(());
                },
            );
            let outcoming = Outcoming {
                application_user_access_token_encoded,
                application_user_access_refresh_token_encoded,
            };
            return Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
