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
                User_Password,
            },
            user_access_refresh_token::UserAccessRefreshToken,
            user_device::UserDevice_Id,
            user_reset_password_token::{
                UserResetPasswordToken,
                UserResetPasswordToken_Value,
                UserResetPasswordToken_WrongEnterTriesQuantity,
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
            if !Validator::<UserResetPasswordToken_Value>::is_valid(incoming.application_user_reset_password_token__value.as_str())? {
                return Result::Err(
                    AggregateError::new_invalid_argument(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
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
            if !Validator::<User_Password>::is_valid_part_1(incoming.application_user__password.as_str()) {
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
            let mut application_user_reset_password_token = match PostgresqlRepository::<UserResetPasswordToken>::find_2(
                database_2_postgresql_connection,
                By1_ {
                    application_user__id: incoming.application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(application_user_reset_password_token_) => application_user_reset_password_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(application_user_reset_password_token.expires_at) {
                PostgresqlRepository::<UserResetPasswordToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1_ {
                        application_user__id: incoming.application_user__id,
                        application_user_device__id: incoming.application_user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_AlreadyExpired));
            }
            if !application_user_reset_password_token.is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_IsNotApproved));
            }
            if application_user_reset_password_token.value != incoming.application_user_reset_password_token__value {
                application_user_reset_password_token.wrong_enter_tries_quantity =
                    application_user_reset_password_token.wrong_enter_tries_quantity.checked_add(1).into_logic_out_of_range(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?;
                if application_user_reset_password_token.wrong_enter_tries_quantity < UserResetPasswordToken_WrongEnterTriesQuantity::LIMIT {
                    PostgresqlRepository::<UserResetPasswordToken>::update_4(
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
                    PostgresqlRepository::<UserResetPasswordToken<'_>>::delete_2(
                        database_2_postgresql_connection,
                        By1_ {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Result::Ok(UnifiedReport::precedent(Precedent::UserResetPasswordToken_WrongValue));
            }
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
            let mut application_user = match PostgresqlRepository::<User>::find_5(
                database_1_postgresql_connection,
                By3 {
                    application_user__id: incoming.application_user__id,
                },
            )
            .await?
            {
                Option::Some(application_user_) => application_user_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::User_NotFound));
                }
            };
            if !Validator::<User_Password>::is_valid_part_2(
                incoming.application_user__password.as_str(),
                application_user.email.as_str(),
                application_user.nickname.as_str(),
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
            let application_user__password_hash___join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<User_Password>::encode(incoming.application_user__password.as_str());
                },
            );
            application_user.password_hash = application_user__password_hash___join_handle.await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )??;
            PostgresqlRepository::<User>::update_1(
                database_1_postgresql_connection,
                Update1 {
                    application_user__password_hash: application_user.password_hash.as_str(),
                },
                By3 {
                    application_user__id: incoming.application_user__id,
                },
            )
            .await?;
            PostgresqlRepository::<UserAccessRefreshToken<'_>>::delete_2(
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
                    PostgresqlRepository::<UserResetPasswordToken<'_>>::delete_2(
                        &*database_2_postgresql_pooled_connection,
                        By1_ {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            return Result::Ok(UnifiedReport::target_empty());
        };
    }
}
