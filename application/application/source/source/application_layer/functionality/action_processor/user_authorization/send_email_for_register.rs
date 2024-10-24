use crate::{
    application_layer::functionality::action_processor::{
        ActionProcessor,
        ActionProcessor_,
        Inner,
    },
    domain_layer::{
        data::entity::{
            user::User_Email,
            user_device::UserDevice_Id,
            user_registration_token::{
                UserRegistrationToken,
                UserRegistrationToken_CanBeResentFrom,
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
            aggregate_error::{
                AggregateError,
                Backtrace,
            },
            capture::Capture,
        },
        functionality::{
            repository::postgresql::{
                user_registration_token::{
                    By1,
                    Update2,
                },
                PostgresqlRepository,
            },
            service::{
                resolver::{
                    Expiration,
                    Resolver,
                },
                spawner::{
                    Spawner,
                    TokioNonBlockingTask,
                },
            },
        },
    },
};
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::send_email_for_register::{
        Incoming,
        Outcoming,
        Precedent,
    },
    unified_report::UnifiedReport,
    void::Void,
};
use std::future::Future;
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
pub struct UserAuthorization_SendEmailForRegister;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_SendEmailForRegister> {
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
            if !Validator::<User_Email>::is_valid(incoming.user__email.as_str())? {
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
            let mut user_registration_token = match PostgresqlRepository::<UserRegistrationToken>::find_3(
                database_2_postgresql_connection,
                By1 {
                    user__email: incoming.user__email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(user_registration_token_) => user_registration_token_,
                Option::None => {
                    return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_NotFound));
                }
            };
            if Resolver::<Expiration>::is_expired(user_registration_token.expires_at) {
                PostgresqlRepository::<UserRegistrationToken<'_>>::delete_2(
                    database_2_postgresql_connection,
                    By1 {
                        user__email: incoming.user__email.as_str(),
                        user_device__id: incoming.user_device__id.as_str(),
                    },
                )
                .await?;
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_AlreadyExpired));
            }
            if user_registration_token.is_approved {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_AlreadyApproved));
            }
            if !Resolver::<Expiration>::is_expired(user_registration_token.can_be_resent_from) {
                return Result::Ok(UnifiedReport::precedent(Precedent::UserRegistrationToken_TimeToResendHasNotCome));
            }
            user_registration_token.can_be_resent_from = Generator::<UserRegistrationToken_CanBeResentFrom>::generate()?;
            PostgresqlRepository::<UserRegistrationToken>::update_2(
                database_2_postgresql_connection,
                Update2 {
                    user_registration_token__can_be_resent_from: user_registration_token.can_be_resent_from,
                },
                By1 {
                    user__email: incoming.user__email.as_str(),
                    user_device__id: incoming.user_device__id.as_str(),
                },
            )
            .await?;
            let environment_configuration_ = inner.environment_configuration;
            Spawner::<TokioNonBlockingTask>::spawn_into_background(
                async move {
                    EmailSender::<UserRegistrationToken<'_>>::repeatable_send(
                        environment_configuration_,
                        user_registration_token.value.as_str(),
                        incoming.user__email.as_str(),
                        incoming.user_device__id.as_str(),
                    )
                    .await?;
                    return Result::Ok(());
                },
            );
            let outcoming = Outcoming {
                user_registration_token__can_be_resent_from: user_registration_token.can_be_resent_from,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
