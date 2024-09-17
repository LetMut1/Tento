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
            application_user_authorization_token::{
                ApplicationUserAuthorizationToken,
                ApplicationUserAuthorizationToken_CanBeResentFrom,
            },
            application_user_device::ApplicationUserDevice_Id,
        },
        functionality::service::{
            email_sender::EmailSender,
            generator::Generator,
            validator::Validator,
        },
    },
    infrastructure_layer::{
        data::capture::Capture,
        functionality::{
            repository::postgresql::{
                application_user::By3,
                application_user_authorization_token::{
                    By1,
                    Update3,
                },
                PostgresqlRepository,
            },
            service::{
                expiration_time_checker::{
                    unix_time::UnixTime,
                    ExpirationTimeChecker,
                },
                spawner::{
                    tokio_non_blocking_task::TokioNonBlockingTask,
                    Spawner,
                },
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_authorize::{
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
pub struct ApplicationUser__Authorization___SendEmailForAuthorize;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___SendEmailForAuthorize> {
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
            let mut application_user_authorization_token = match PostgresqlRepository::<ApplicationUserAuthorizationToken>::find_3(
                database_2_postgresql_connection,
                By1 {
                    application_user__id: incoming.application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Some(application_user_authorization_token_) => application_user_authorization_token_,
                None => {
                    return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_NotFound));
                }
            };
            if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token.expires_at) {
                PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::delete_1(
                    database_2_postgresql_connection,
                    By1 {
                        application_user__id: incoming.application_user__id,
                        application_user_device__id: incoming.application_user_device__id.as_str(),
                    },
                )
                .await?;
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_AlreadyExpired));
            }
            if !ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token.can_be_resent_from) {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUserAuthorizationToken_TimeToResendHasNotCome));
            }
            application_user_authorization_token.can_be_resent_from = Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate()?;
            PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_3(
                database_2_postgresql_connection,
                Update3 {
                    application_user_authorization_token__can_be_resent_from: application_user_authorization_token.can_be_resent_from,
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
                    EmailSender::<ApplicationUserAuthorizationToken<'_>>::send(
                        environment_configuration_,
                        application_user_authorization_token.value.as_str(),
                        application_user.email.as_str(),
                        incoming.application_user_device__id.as_str(),
                    )
                    .await?;
                    return Ok(());
                },
            );
            let outcoming = Outcoming {
                application_user_authorization_token__can_be_resent_from: application_user_authorization_token.can_be_resent_from,
            };
            return Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
