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
            application_user_authorization_token::{
                ApplicationUserAuthorizationToken,
                ApplicationUserAuthorizationToken_CanBeResentFrom,
                ApplicationUserAuthorizationToken_ExpiresAt,
                ApplicationUserAuthorizationToken_Value,
                ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
            },
            application_user_device::ApplicationUserDevice_Id,
        },
        functionality::service::{
            email_sender::EmailSender,
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
                },
                application_user_authorization_token::{
                    By1 as By1_,
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
                    tokio_blocking_task::TokioBlockingTask,
                    tokio_non_blocking_task::TokioNonBlockingTask,
                    Spawner,
                },
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::{
    Incoming,
    Outcoming,
    Precedent,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
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
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep;
impl ActionProcessor_ for ActionProcessor<ApplicationUser__Authorization___AuthorizeByFirstStep> {
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
            let database_1_postgresql_pooled_connection = inner.get_database_1_postgresql_pooled_connection().await?;
            let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
            let (application_user__id, application_user__email, application_user__nickname, application_user__password_hash) =
                if Validator::<ApplicationUser_Email>::is_valid(incoming.application_user__email___or___application_user__nickname.as_str())? {
                    let application_user_ = PostgresqlRepository::<ApplicationUser>::find_3(
                        database_1_postgresql_connection,
                        By2 {
                            application_user__email: incoming.application_user__email___or___application_user__nickname.as_str(),
                        },
                    )
                    .await?;
                    let application_user__ = match application_user_ {
                        Some(application_user___) => application_user___,
                        None => {
                            return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword));
                        }
                    };
                    (
                        application_user__.id,
                        incoming.application_user__email___or___application_user__nickname,
                        application_user__.nickname,
                        application_user__.password_hash,
                    )
                } else {
                    if Validator::<ApplicationUser_Nickname>::is_valid(incoming.application_user__email___or___application_user__nickname.as_str()) {
                        let application_user_ = PostgresqlRepository::<ApplicationUser>::find_2(
                            database_1_postgresql_connection,
                            By1 {
                                application_user__nickname: incoming.application_user__email___or___application_user__nickname.as_str(),
                            },
                        )
                        .await?;
                        let application_user__ = match application_user_ {
                            Some(application_user___) => application_user___,
                            None => {
                                return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword));
                            }
                        };
                        (
                            application_user__.id,
                            application_user__.email,
                            incoming.application_user__email___or___application_user__nickname,
                            application_user__.password_hash,
                        )
                    } else {
                        return Err(
                            AggregateError::new_invalid_argument(
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                };
            if !Validator::<ApplicationUser_Password>::is_valid_part_2(
                incoming.application_user__password.as_str(),
                application_user__email.as_str(),
                application_user__nickname.as_str(),
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
            let is_valid_join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
                move || -> _ {
                    return Encoder::<ApplicationUser_Password>::is_valid(
                        incoming.application_user__password.as_str(),
                        application_user__password_hash.as_str(),
                    );
                },
            );
            if !is_valid_join_handle.await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?? {
                return Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword));
            }
            let database_2_postgresql_pooled_connection = inner.get_database_2_postgresql_pooled_connection().await?;
            let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;
            let (
                application_user_authorization_token__value,
                application_user_authorization_token__can_be_resent_from,
                application_user_authorization_token__wrong_enter_tries_quantity,
                can_send,
            ) = match PostgresqlRepository::<ApplicationUserAuthorizationToken>::find_1(
                database_2_postgresql_connection,
                By1_ {
                    application_user__id,
                    application_user_device__id: incoming.application_user_device__id.as_str(),
                },
            )
            .await?
            {
                Some(mut application_user_authorization_token) => {
                    let (can_send_, need_to_update_1) = if Resolver::<Expiration>::is_expired(application_user_authorization_token.can_be_resent_from) {
                        application_user_authorization_token.can_be_resent_from = Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate()?;
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
                    let need_to_update_2 = if Resolver::<Expiration>::is_expired(application_user_authorization_token.expires_at) {
                        application_user_authorization_token.value = Generator::<ApplicationUserAuthorizationToken_Value>::generate();
                        application_user_authorization_token.wrong_enter_tries_quantity = 0;
                        application_user_authorization_token.expires_at = Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate()?;
                        true
                    } else {
                        false
                    };
                    if need_to_update_1 && need_to_update_2 {
                        PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_1(
                            database_2_postgresql_connection,
                            Update1 {
                                application_user_authorization_token__value: application_user_authorization_token.value.as_str(),
                                application_user_authorization_token__wrong_enter_tries_quantity: application_user_authorization_token.wrong_enter_tries_quantity,
                                application_user_authorization_token__expires_at: application_user_authorization_token.expires_at,
                                application_user_authorization_token__can_be_resent_from: application_user_authorization_token.can_be_resent_from,
                            },
                            By1_ {
                                application_user__id,
                                application_user_device__id: incoming.application_user_device__id.as_str(),
                            },
                        )
                        .await?;
                    } else {
                        if need_to_update_1 {
                            PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_3(
                                database_2_postgresql_connection,
                                Update3 {
                                    application_user_authorization_token__can_be_resent_from: application_user_authorization_token.can_be_resent_from,
                                },
                                By1_ {
                                    application_user__id,
                                    application_user_device__id: incoming.application_user_device__id.as_str(),
                                },
                            )
                            .await?;
                        }
                        if need_to_update_2 {
                            PostgresqlRepository::<ApplicationUserAuthorizationToken>::update_2(
                                database_2_postgresql_connection,
                                Update2 {
                                    application_user_authorization_token__value: application_user_authorization_token.value.as_str(),
                                    application_user_authorization_token__wrong_enter_tries_quantity: application_user_authorization_token.wrong_enter_tries_quantity,
                                    application_user_authorization_token__expires_at: application_user_authorization_token.expires_at,
                                },
                                By1_ {
                                    application_user__id,
                                    application_user_device__id: incoming.application_user_device__id.as_str(),
                                },
                            )
                            .await?;
                        }
                    }
                    (
                        application_user_authorization_token.value,
                        application_user_authorization_token.can_be_resent_from,
                        application_user_authorization_token.wrong_enter_tries_quantity,
                        can_send_,
                    )
                }
                None => {
                    let application_user_authorization_token = PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::create_1(
                        database_2_postgresql_connection,
                        Insert1 {
                            application_user__id,
                            application_user_device__id: incoming.application_user_device__id.as_str(),
                            application_user_authorization_token__value: Generator::<ApplicationUserAuthorizationToken_Value>::generate(),
                            application_user_authorization_token__wrong_enter_tries_quantity: 0,
                            application_user_authorization_token__expires_at: Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate()?,
                            application_user_authorization_token__can_be_resent_from: Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate()?,
                        },
                    )
                    .await?;
                    (
                        application_user_authorization_token.value,
                        application_user_authorization_token.can_be_resent_from,
                        application_user_authorization_token.wrong_enter_tries_quantity,
                        true,
                    )
                }
            };
            if can_send {
                let environment_configuration_ = inner.environment_configuration;
                Spawner::<TokioNonBlockingTask>::spawn_into_background(
                    async move {
                        EmailSender::<ApplicationUserAuthorizationToken<'_>>::send(
                            environment_configuration_,
                            application_user_authorization_token__value.as_str(),
                            application_user__email.as_str(),
                            incoming.application_user_device__id.as_str(),
                        )
                        .await?;
                        return Ok(());
                    },
                );
            }
            let outcoming = Outcoming {
                application_user__id,
                verification_message_sent: can_send,
                application_user_authorization_token__can_be_resent_from,
                application_user_authorization_token__wrong_enter_tries_quantity,
                application_user_authorization_token__wrong_enter_tries_quantity_limit: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::LIMIT,
            };
            return Ok(UnifiedReport::target_filled(outcoming));
        };
    }
}
