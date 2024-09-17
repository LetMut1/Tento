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
                ApplicationUser_Id,
                ApplicationUser_Password,
            },
            application_user_access_refresh_token::ApplicationUserAccessRefreshToken,
            application_user_device::ApplicationUserDevice_Id,
            application_user_reset_password_token::{
                ApplicationUserResetPasswordToken,
                ApplicationUserResetPasswordToken_Value,
                ApplicationUserResetPasswordToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::{
            encoder::Encoder,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::{
            repository::postgresql::{
                application_user::{
                    By3,
                    Update1,
                },
                application_user_access_refresh_token::By1,
                application_user_reset_password_token::{
                    By1 as By1_,
                    Update4,
                },
                PostgresqlRepository,
            },
            service::{
                resolver::{
                    cloud_message::CloudMessage,
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
use crate::infrastructure_layer::functionality::service::resolver::expiration::Expiration;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_last_step::{
    Incoming,
    Precedent,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    OptionConverter,
    ResultConverter,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub struct ApplicationUser__Authorization___ResetPasswordByLastStep;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___ResetPasswordByLastStep> {
    type Incoming = Incoming;
    type Outcoming = Void;
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
            if !Validator::<ApplicationUserResetPasswordToken_Value>::is_valid(incoming.application_user_reset_password_token__value.as_str())? {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUser_Id>::is_valid(incoming.application_user__id) {
                return Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUser_Password>::is_valid_part_1(incoming.application_user__password.as_str()) {
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
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken>::find_2(
                database_2_postgresql_connection,
                By1_ {
                    application_user__id: incoming.application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Some(application_user_reset_password_token_) => application_user_reset_password_token_,
                None => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(application_user_reset_password_token.expires_at) {
                PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1_ {
                        application_user__id: incoming.application_user__id,
                        application_user_device__id: incoming.application_user_device__id.as_str(),
                    },
                )
                .await?;
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyExpired));
            }
            if !application_user_reset_password_token.is_approved {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_IsNotApproved));
            }
            if application_user_reset_password_token.value != incoming.application_user_reset_password_token__value {
                application_user_reset_password_token.wrong_enter_tries_quantity =
                    application_user_reset_password_token.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                if application_user_reset_password_token.wrong_enter_tries_quantity < ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                    PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_4(
                        database_2_postgresql_connection,
                        Update4 {
                            application_user_reset_password_token__wrong_enter_tries_quantity: application_user_reset_password_token.wrong_enter_tries_quantity,
                        },
                        By1_ {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                        database_2_postgresql_connection,
                        By1_ {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_WrongValue));
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
            let mut application_user = match PostgresqlRepository::<ApplicationUser>::find_5(
                database_1_postgresql_connection,
                By3 {
                    application_user__id: incoming.application_user__id,
                },
            )
            .await?
            {
                Some(application_user_) => application_user_,
                None => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_NotFound));
                }
            };
            if !Validator::<ApplicationUser_Password>::is_valid_part_2(
                incoming.application_user__password.as_str(),
                application_user.email.as_str(),
                application_user.nickname.as_str(),
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
            let application_user__password_hash___join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<ApplicationUser_Password>::encode(incoming.application_user__password.as_str());
                },
            );
            application_user.password_hash = application_user__password_hash___join_handle.await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )??;
            PostgresqlRepository::<ApplicationUser>::update_1(
                database_1_postgresql_connection,
                Update1 {
                    application_user__password_hash: application_user.password_hash.as_str(),
                },
                By3 {
                    application_user__id: incoming.application_user__id,
                },
            )
            .await?;
            PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_2(
                &*database_2_postgresql_pooled_connection,
                By1 {
                    application_user__id: incoming.application_user__id,
                },
            )
            .await?;
            Resolver::<CloudMessage>::deauthorize_application_user_from_all_devices();
            let database_2_postgresql_connection_pool = inner.database_2_postgresql_connection_pool.clone();
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.into_runtime(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                    PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                        &*database_2_postgresql_pooled_connection,
                        By1_ {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                    return Ok(());
                },
            );
            return Ok(UnifiedReport::target_empty());
        };
    }
}
