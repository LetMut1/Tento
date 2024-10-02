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
                User_Email,
            },
            user_device::UserDevice_Id,
            user_registration_token::{
                UserRegistrationToken,
                UserRegistrationToken_CanBeResentFrom,
                UserRegistrationToken_ExpiresAt,
                UserRegistrationToken_Value,
                UserRegistrationToken_WrongEnterTriesQuantity,
            },
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
                application_user::By2,
                application_user_registration_token::{
                    By1,
                    Insert1,
                    Update1,
                    Update2,
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
use action_processor_incoming_outcoming::action_processor::user_authorization::register_by_first_step::{
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
pub struct UserAuthorization_RegisterByFirstStep;
impl ActionProcessor_ for ActionProcessor<UserAuthorization_RegisterByFirstStep> {
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
            if !Validator::<User_Email>::is_valid(incoming.application_user__email.as_str())? {
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
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            if PostgresqlRepository::<User<'_>>::is_exist_2(
                &*database_1_postgresql_pooled_connection,
                By2 {
                    application_user__email: incoming.application_user__email.as_str(),
                },
            )
            .await?
            {
                return Result::Ok(UnifiedReport::precedent(Precedent::User_EmailAlreadyExist));
            }
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let (
                application_user_registration_token__value,
                application_user_registration_token__can_be_resent_from,
                application_user_registration_token__wrong_enter_tries_quantity,
                can_send,
            ) = match PostgresqlRepository::<UserRegistrationToken>::find_1(
                database_2_postgresql_connection,
                By1 {
                    application_user__email: incoming.application_user__email.as_str(),
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Option::Some(mut application_user_registration_token) => {
                    let (can_send_, need_to_update_1) = if Resolver::<Expiration>::is_expired(application_user_registration_token.can_be_resent_from) {
                        application_user_registration_token.can_be_resent_from = Generator::<UserRegistrationToken_CanBeResentFrom>::generate()?;
                        (
                            true,
                            true,
                        )
                    } else {
                        (
                            false,
                            false,
                        )
                    };
                    let need_to_update_2 = if Resolver::<Expiration>::is_expired(application_user_registration_token.expires_at) || application_user_registration_token.is_approved
                    {
                        application_user_registration_token.value = Generator::<UserRegistrationToken_Value>::generate();
                        application_user_registration_token.wrong_enter_tries_quantity = 0;
                        application_user_registration_token.is_approved = false;
                        application_user_registration_token.expires_at = Generator::<UserRegistrationToken_ExpiresAt>::generate()?;
                        true
                    } else {
                        false
                    };
                    if need_to_update_1 && need_to_update_2 {
                        PostgresqlRepository::<UserRegistrationToken>::update_1(
                            database_2_postgresql_connection,
                            Update1 {
                                application_user_registration_token__value: application_user_registration_token.value.as_str(),
                                application_user_registration_token__wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                                application_user_registration_token__is_approved: application_user_registration_token.is_approved,
                                application_user_registration_token__expires_at: application_user_registration_token.expires_at,
                                application_user_registration_token__can_be_resent_from: application_user_registration_token.can_be_resent_from,
                            },
                            By1 {
                                application_user__email: incoming.application_user__email.as_str(),
                                application_user_device__id: incoming.application_user_device__id.as_str(),
                            },
                        )
                        .await?;
                    } else {
                        if need_to_update_1 {
                            PostgresqlRepository::<UserRegistrationToken>::update_2(
                                database_2_postgresql_connection,
                                Update2 {
                                    application_user_registration_token__can_be_resent_from: application_user_registration_token.can_be_resent_from,
                                },
                                By1 {
                                    application_user__email: incoming.application_user__email.as_str(),
                                    application_user_device__id: incoming.application_user_device__id.as_str(),
                                },
                            )
                            .await?;
                        }
                        if need_to_update_2 {
                            PostgresqlRepository::<UserRegistrationToken>::update_3(
                                database_2_postgresql_connection,
                                Update3 {
                                    application_user_registration_token__value: application_user_registration_token.value.as_str(),
                                    application_user_registration_token__wrong_enter_tries_quantity: application_user_registration_token.wrong_enter_tries_quantity,
                                    application_user_registration_token__is_approved: application_user_registration_token.is_approved,
                                    application_user_registration_token__expires_at: application_user_registration_token.expires_at,
                                },
                                By1 {
                                    application_user__email: incoming.application_user__email.as_str(),
                                    application_user_device__id: incoming.application_user_device__id.as_str(),
                                },
                            )
                            .await?;
                        }
                    }
                    (
                        application_user_registration_token.value,
                        application_user_registration_token.can_be_resent_from,
                        application_user_registration_token.wrong_enter_tries_quantity,
                        can_send_,
                    )
                }
                Option::None => {
                    let application_user_registration_token = PostgresqlRepository::<UserRegistrationToken<'_>>::create_1(
                        database_2_postgresql_connection,
                        Insert1 {
                            application_user__email: incoming.application_user__email.as_str(),
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                            application_user_registration_token__value: Generator::<UserRegistrationToken_Value>::generate(),
                            application_user_registration_token__wrong_enter_tries_quantity: 0,
                            application_user_registration_token__is_approved: false,
                            application_user_registration_token__expires_at: Generator::<UserRegistrationToken_ExpiresAt>::generate()?,
                            application_user_registration_token__can_be_resent_from: Generator::<UserRegistrationToken_CanBeResentFrom>::generate()?,
                        },
                    )
                    .await?;
                    (
                        application_user_registration_token.value,
                        application_user_registration_token.can_be_resent_from,
                        application_user_registration_token.wrong_enter_tries_quantity,
                        true,
                    )
                }
            };
            if can_send {
                let environment_configuration_ = inner.environment_configuration;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        EmailSender::<UserRegistrationToken<'_>>::repeatable_send(
                            environment_configuration_,
                            application_user_registration_token__value.as_str(),
                            incoming.application_user__email.as_str(),
                            incoming.application_user_device__id.as_str(),
                        )
                        .await?;
                        return Result::Ok(());
                    },
                );
            }
            let outcoming = Outcoming {
                verification_message_sent: can_send,
                application_user_registration_token__can_be_resent_from,
                application_user_registration_token__wrong_enter_tries_quantity,
                application_user_registration_token__wrong_enter_tries_quantity_limit: UserRegistrationToken_WrongEnterTriesQuantity::LIMIT,
            };
            return Result::Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
