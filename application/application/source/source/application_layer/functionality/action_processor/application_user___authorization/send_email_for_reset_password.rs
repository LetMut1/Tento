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
            },
            application_user_device::ApplicationUserDevice_Id,
            application_user_reset_password_token::{
                ApplicationUserResetPasswordToken,
                ApplicationUserResetPasswordToken_CanBeResentFrom,
            },
        },
        functionality::service::{
            email_sender::EmailSender,
            generator::Generator,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::{
            capture::Capture,
            control_type::{
                ApplicationUser__Authorization___SendEmailForResetPassword, TokioNonBlockingTask, UnixTime
            },
        },
        functionality::{
            repository::postgresql::{
                application_user::By3,
                application_user_reset_password_token::{
                    By1,
                    Update2,
                },
                PostgresqlRepository,
            },
            service::{expiration_time_checker::ExpirationTimeChecker, spawner::Spawner},
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_reset_password::{
    Incoming,
    Outcoming,
    Precedent,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
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
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___SendEmailForResetPassword> {
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
            let application_user = match PostgresqlRepository::<ApplicationUser>::find_6(
                &*database_1_postgresql_pooled_connection,
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
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken>::find_3(
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
            if !ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token.can_be_resent_from) {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_TimeToResendHasNotCome));
            }
            application_user_reset_password_token.can_be_resent_from = Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate()?;
            PostgresqlRepository::<ApplicationUserResetPasswordToken>::update_2(
                database_2_postgresql_connection,
                Update2 {
                    application_user_reset_password_token__can_be_resent_from: application_user_reset_password_token.can_be_resent_from,
                },
                By1 {
                    application_user__id: incoming.application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?;
            let environment_configuration_ = inner.environment_configuration;
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    EmailSender::<ApplicationUserResetPasswordToken<'_>>::send(
                        environment_configuration_,
                        application_user_reset_password_token.value.as_str(),
                        application_user.email.as_str(),
                        incoming.application_user_device__id.as_str(),
                    ).await?;
                    return Ok(());
                }
            );
            let outcoming = Outcoming {
                application_user_reset_password_token__can_be_resent_from: application_user_reset_password_token.can_be_resent_from,
            };
            return Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
