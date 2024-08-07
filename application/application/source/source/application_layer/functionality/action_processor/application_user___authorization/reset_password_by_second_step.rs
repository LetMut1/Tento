use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::{
            application_user::ApplicationUser_Id,
            application_user_device::ApplicationUserDevice_Id,
            application_user_reset_password_token::{
                ApplicationUserResetPasswordToken,
                ApplicationUserResetPasswordToken_Value,
                ApplicationUserResetPasswordToken_WrongEnterTriesQuantity,
            },
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                OptionConverter,
            },
            control_type::{
                ApplicationUser__Authorization___ResetPasswordBySecondStep,
                UnixTime,
            },
            void::Void,
        },
        functionality::{
            repository::postgresql::{
                application_user_reset_password_token::{
                    By1,
                    Update4,
                    Update5,
                },
                PostgresqlRepository,
            },
            service::expiration_time_checker::ExpirationTimeChecker,
        },
    },
};
use crate::infrastructure_layer::data::capture::Capture;
use crate::application_layer::functionality::action_processor::Inner;
use crate::application_layer::functionality::action_processor::ActionProcessor_;
use std::future::Future;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_second_step::{
    Incoming,
    Precedent,
};
use std::{
    clone::Clone,
    marker::{
        Send,
        Sync,
    },
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___ResetPasswordBySecondStep> {
    type Incoming = Incoming;
    type Outcoming = Void;
    type Precedent = Precedent;
    fn process<'a, T> (
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
                    AggregateError::new_invalid_argument_from_outside(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUser_Id>::is_valid(incoming.application_user__id) {
                return Err(
                    AggregateError::new_invalid_argument_from_outside(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming.application_user_device__id.as_str()) {
                return Err(
                    AggregateError::new_invalid_argument_from_outside(
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
                By1 {
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
            if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token.expires_at) {
                PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1 {
                        application_user__id: incoming.application_user__id,
                        application_user_device__id: incoming.application_user_device__id.as_str(),
                    },
                )
                .await?;
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyExpired));
            }
            if application_user_reset_password_token.is_approved {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyApproved));
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
                        By1 {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                } else {
                    PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete_2(
                        database_2_postgresql_connection,
                        By1 {
                            application_user__id: incoming.application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                        },
                    )
                    .await?;
                }
                return Ok(
                    UnifiedReport::precedent(
                        Precedent::ApplicationUserResetPasswordToken_WrongValue {
                            application_user_reset_password_token__wrong_enter_tries_quantity: application_user_reset_password_token.wrong_enter_tries_quantity,
                        },
                    ),
                );
            }
            application_user_reset_password_token.is_approved = true;
            PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_5(
                database_2_postgresql_connection,
                Update5 {
                    application_user_reset_password_token__is_approved: application_user_reset_password_token.is_approved,
                },
                By1 {
                    application_user__id: incoming.application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?;
            return Ok(UnifiedReport::target_empty());
        };
    }
}
