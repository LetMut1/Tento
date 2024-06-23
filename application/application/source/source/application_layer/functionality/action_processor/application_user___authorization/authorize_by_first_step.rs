use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
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
        data::{
            auditor::{
                Auditor,
                Backtrace,
                OptionConverter,
                ResultConverter,
            },
            control_type::{
                ApplicationUser__Authorization___AuthorizeByFirstStep,
                TokioBlockingTask,
                UnixTime,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
            invalid_argument::InvalidArgument,
        },
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
                expiration_time_checker::ExpirationTimeChecker,
                spawner::Spawner,
            },
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::{
    Incoming,
    Outcoming,
    Precedent,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
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
impl ActionProcessor<ApplicationUser__Authorization___AuthorizeByFirstStep> {
    pub async fn process<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<Result<UnifiedReport<Outcoming, Precedent>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        if !Validator::<ApplicationUser_Password>::is_valid_part_1(incoming_.application_user_password.as_str()) {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }
        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming_.application_user_device__id.as_str()) {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;
        let (application_user__id, application_user__email, application_user__nickname, application_user__password_hash) =
            if Validator::<ApplicationUser_Email>::is_valid(incoming_.application_user__email___or___application_user__nickname.as_str())? {
                let application_user_ = PostgresqlRepository::<ApplicationUser>::find_3(
                    database_1_postgresql_connection,
                    By2 {
                        application_user__email: incoming_.application_user__email___or___application_user__nickname.as_str(),
                    },
                )
                .await?;
                let application_user__ = match application_user_ {
                    Some(application_user___) => application_user___,
                    None => {
                        return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword)));
                    }
                };
                (
                    application_user__.id,
                    incoming_.application_user__email___or___application_user__nickname,
                    application_user__.nickname,
                    application_user__.password_hash,
                )
            } else {
                if Validator::<ApplicationUser_Nickname>::is_valid(incoming_.application_user__email___or___application_user__nickname.as_str()) {
                    let application_user_ = PostgresqlRepository::<ApplicationUser>::find_2(
                        database_1_postgresql_connection,
                        By1 {
                            application_user__nickname: incoming_.application_user__email___or___application_user__nickname.as_str(),
                        },
                    )
                    .await?;
                    let application_user__ = match application_user_ {
                        Some(application_user___) => application_user___,
                        None => {
                            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword)));
                        }
                    };
                    (
                        application_user__.id,
                        application_user__.email,
                        incoming_.application_user__email___or___application_user__nickname,
                        application_user__.password_hash,
                    )
                } else {
                    return Ok(
                        Err(
                            Auditor::<InvalidArgument>::new(
                                InvalidArgument,
                                Backtrace::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        ),
                    );
                }
            };
        if !Validator::<ApplicationUser_Password>::is_valid_part_2(
            incoming_.application_user_password.as_str(),
            application_user__email.as_str(),
            application_user__nickname.as_str(),
        ) {
            return Ok(
                Err(
                    Auditor::<InvalidArgument>::new(
                        InvalidArgument,
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                ),
            );
        }
        let join_handle = Spawner::<TokioBlockingTask>::spawn_processed(
            move || -> _ {
                return Encoder::<ApplicationUser_Password>::is_valid(
                    incoming_.application_user_password.as_str(),
                    application_user__password_hash.as_str(),
                );
            },
        );
        if !join_handle.await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?? {
            return Ok(Ok(UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword)));
        }
        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert_into_error(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
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
                application_user_device__id: incoming_.application_user_device__id.as_str(),
            },
        )
        .await?
        {
            Some(mut application_user_authorization_token) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token.can_be_resent_from) {
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
                let need_to_update_2 = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token.expires_at) {
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
                            application_user_device__id: incoming_.application_user_device__id.as_str(),
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
                                application_user_device__id: incoming_.application_user_device__id.as_str(),
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
                                application_user_device__id: incoming_.application_user_device__id.as_str(),
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
                        application_user_device__id: incoming_.application_user_device__id.as_str(),
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
            EmailSender::<ApplicationUserAuthorizationToken<'_>>::send(
                environment_configuration,
                application_user_authorization_token__value.as_str(),
                application_user__email.as_str(),
                incoming_.application_user_device__id.as_str(),
            )?;
        }
        let outcoming = Outcoming {
            application_user__id,
            verification_message_sent: can_send,
            application_user_authorization_token__can_be_resent_from,
            application_user_authorization_token__wrong_enter_tries_quantity,
            application_user_authorization_token__wrong_enter_tries_quantity_limit: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::LIMIT,
        };
        return Ok(Ok(UnifiedReport::target_filled(outcoming)));
    }
}
