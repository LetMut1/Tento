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
            user_authorization_token::{
                UserAuthorizationToken,
                UserAuthorizationToken_CanBeResentFrom,
            },
            user_device::UserDevice_Id,
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
use action_processor_incoming_outcoming::action_processor::user_authorization::send_email_for_authorize::{
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
pub struct UserAuthorization_SendEmailForAuthorize;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_SendEmailForAuthorize> {
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
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let application_user = match PostgresqlRepository::<User>::find_6(
                &*database_1_postgresql_pooled_connection,
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
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let mut application_user_authorization_token = match PostgresqlRepository::<UserAuthorizationToken>::find_3(
                database_2_postgresql_connection,
                By1 {
                    application_user__id: incoming.application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(application_user_authorization_token_) => application_user_authorization_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(application_user_authorization_token.expires_at) {
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
            if !Resolver::<Expiration>::is_expired(application_user_authorization_token.can_be_resent_from) {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserAuthorizationToken_TimeToResendHasNotCome));
            }
            application_user_authorization_token.can_be_resent_from = Generator::<UserAuthorizationToken_CanBeResentFrom>::generate()?;
            PostgresqlRepository::<UserAuthorizationToken>::update_3(
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
                    EmailSender::<UserAuthorizationToken<'_>>::repeatable_send(
                        environment_configuration_,
                        application_user_authorization_token.value.as_str(),
                        application_user.email.as_str(),
                        incoming.application_user_device__id.as_str(),
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                application_user_authorization_token__can_be_resent_from: application_user_authorization_token.can_be_resent_from,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}