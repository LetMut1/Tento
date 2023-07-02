use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser1;
use crate::domain_layer::data::entity::application_user::ApplicationUser2;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken1;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken2;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken3;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::functionality::service::email_sender::EmailSender;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::pushable_environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token___postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use extern_crate::bb8::Pool;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::macro_rules::r#enum;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio_postgres::Socket;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        if !Validator::<ApplicationUser_Password>::is_valid(&incoming.application_user_password) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Password,
                },
            );
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(&incoming.application_user_device_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserDevice_Id,
                },
            );
        }

        let application_user_email = ApplicationUser_Email::new(incoming.application_user_email_or_application_user_nickname);

        let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(&application_user_email) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        let application_user_aggregator = if is_valid_email {
            let application_user_ = match PostgresqlRepository::<ApplicationUser2>::find_2(
                database_1_postgresql_connection,
                &application_user_email,
            )
            .await
            {
                Ok(application_user__) => application_user__,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

            let application_user__ = match application_user_ {
                Some(application_user___) => application_user___,
                None => {
                    return Ok(
                        InvalidArgumentResult::Ok {
                            subject: UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword),
                        },
                    );
                }
            };

            ApplicationUser_Aggregator::Second {
                application_user: application_user__,
                application_user_email,
            }
        } else {
            let application_user_nickname = ApplicationUser_Nickname::new(application_user_email.into_inner());

            if Validator::<ApplicationUser_Nickname>::is_valid(&application_user_nickname) {
                let application_user_ = match PostgresqlRepository::<ApplicationUser1>::find_1(
                    database_1_postgresql_connection,
                    &application_user_nickname,
                )
                .await
                {
                    Ok(application_user__) => application_user__,
                    Err(mut error) => {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        );

                        return Err(error);
                    }
                };

                let application_user__ = match application_user_ {
                    Some(application_user___) => application_user___,
                    None => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword),
                            },
                        );
                    }
                };

                ApplicationUser_Aggregator::First {
                    application_user: application_user__,
                }
            } else {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument: InvalidArgument::ApplicationUser_Nickname,
                    },
                );
            }
        };

        let application_user_password_hash = match application_user_aggregator {
            ApplicationUser_Aggregator::First {
                ref application_user,
            } => application_user.get_password_hash(),
            ApplicationUser_Aggregator::Second {
                ref application_user,
                application_user_email: _,
            } => application_user.get_password_hash(),
        };

        let is_valid = match Encoder::<ApplicationUser_Password>::is_valid(
            &incoming.application_user_password,
            application_user_password_hash,
        ) {
            Ok(is_valid_) => is_valid_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        if !is_valid {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUser_WrongEmailOrNicknameOrPassword),
                },
            );
        }

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let application_user_id = match application_user_aggregator {
            ApplicationUser_Aggregator::First {
                ref application_user,
            } => application_user.get_id(),
            ApplicationUser_Aggregator::Second {
                ref application_user,
                application_user_email: _,
            } => application_user.get_id(),
        };

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_authorization_token = match PostgresqlRepository::<ApplicationUserAuthorizationToken1>::find_1(
            database_2_postgresql_connection,
            application_user_id,
            &incoming.application_user_device_id,
        )
        .await
        {
            Ok(application_user_authorization_token_) => application_user_authorization_token_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let (application_user_authorization_token_aggregator, can_send) = match application_user_authorization_token {
            Some(mut application_user_authorization_token_) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token_.get_can_be_resent_from().get()) {
                    let application_user_authorization_token_can_be_resent_from = match Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate() {
                        Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
                        Err(mut error) => {
                            error.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                    None,
                                ),
                            );

                            return Err(error);
                        }
                    };

                    application_user_authorization_token_.set_can_be_resent_from(application_user_authorization_token_can_be_resent_from);

                    (
                        true, true,
                    )
                } else {
                    (
                        false, false,
                    )
                };

                let need_to_update_2 = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token_.get_expires_at().get()) {
                    let application_user_authorization_token_expires_at = match Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate() {
                        Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
                        Err(mut error) => {
                            error.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                    None,
                                ),
                            );

                            return Err(error);
                        }
                    };

                    application_user_authorization_token_
                        .set_value(Generator::<ApplicationUserAuthorizationToken_Value>::generate())
                        .set_wrong_enter_tries_quantity(ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::new(0))
                        .set_expires_at(application_user_authorization_token_expires_at);

                    true
                } else {
                    false
                };

                if need_to_update_1 && need_to_update_2 {
                    if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken1>::update(
                        database_2_postgresql_connection,
                        &application_user_authorization_token_,
                        application_user_id,
                        &incoming.application_user_device_id,
                    )
                    .await
                    {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        );

                        return Err(error);
                    }
                } else {
                    if need_to_update_1 {
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken3>::update(
                            database_2_postgresql_connection,
                            &application_user_authorization_token_,
                            application_user_id,
                            &incoming.application_user_device_id,
                        )
                        .await
                        {
                            error.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                    None,
                                ),
                            );

                            return Err(error);
                        }
                    }

                    if need_to_update_2 {
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken2>::update(
                            database_2_postgresql_connection,
                            &application_user_authorization_token_,
                            application_user_id,
                            &incoming.application_user_device_id,
                        )
                        .await
                        {
                            error.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                    None,
                                ),
                            );

                            return Err(error);
                        }
                    }
                }

                (
                    ApplicationUserAuthorizationToken_Aggregator::Second {
                        application_user_authorization_token: application_user_authorization_token_,
                    },
                    can_send_,
                )
            }
            None => {
                let application_user_authorization_token_expires_at = match Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate() {
                    Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
                    Err(mut error) => {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        );

                        return Err(error);
                    }
                };

                let application_user_authorization_token_can_be_resent_from = match Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate() {
                    Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
                    Err(mut error) => {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        );

                        return Err(error);
                    }
                };

                let insert = Insert {
                    application_user_id,
                    application_user_device_id: &incoming.application_user_device_id,
                    application_user_authorization_token_value: Generator::<ApplicationUserAuthorizationToken_Value>::generate(),
                    application_user_authorization_token_wrong_enter_tries_quantity: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::new(0),
                    application_user_authorization_token_expires_at,
                    application_user_authorization_token_can_be_resent_from,
                };

                let application_user_authorization_token_ = match PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::create(
                    database_2_postgresql_connection,
                    insert,
                )
                .await
                {
                    Ok(application_user_authorization_token__) => application_user_authorization_token__,
                    Err(mut error) => {
                        error.add_backtrace_part(
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        );

                        return Err(error);
                    }
                };

                (
                    ApplicationUserAuthorizationToken_Aggregator::First {
                        application_user_authorization_token: application_user_authorization_token_,
                    },
                    true,
                )
            }
        };

        let application_user_email = match application_user_aggregator {
            ApplicationUser_Aggregator::First {
                ref application_user,
            } => application_user.get_email(),
            ApplicationUser_Aggregator::Second {
                application_user: _,
                application_user_email: ref application_user_email_,
            } => application_user_email_,
        };

        if can_send {
            let application_user_authorization_token_value = match application_user_authorization_token_aggregator {
                ApplicationUserAuthorizationToken_Aggregator::First {
                    application_user_authorization_token: ref application_user_authorization_token_,
                } => application_user_authorization_token_.get_value(),
                ApplicationUserAuthorizationToken_Aggregator::Second {
                    application_user_authorization_token: ref application_user_authorization_token_,
                } => application_user_authorization_token_.get_value(),
            };

            if let Err(mut error) = EmailSender::<ApplicationUserAuthorizationToken<'_>>::send(
                pushable_environment_configuration,
                application_user_authorization_token_value,
                application_user_email,
                &incoming.application_user_device_id,
            ) {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        }

        let application_user_authorization_token_can_be_resent_from = match application_user_authorization_token_aggregator {
            ApplicationUserAuthorizationToken_Aggregator::First {
                application_user_authorization_token: ref application_user_authorization_token_,
            } => application_user_authorization_token_.get_can_be_resent_from(),
            ApplicationUserAuthorizationToken_Aggregator::Second {
                application_user_authorization_token: ref application_user_authorization_token_,
            } => application_user_authorization_token_.get_can_be_resent_from(),
        };

        let outcoming = Outcoming {
            application_user_id,
            verification_message_sent: can_send,
            application_user_authorization_token_can_be_resent_from,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(outcoming),
            },
        );
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_device_id: ApplicationUserDevice_Id,
    application_user_email_or_application_user_nickname: String,
    application_user_password: ApplicationUser_Password,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_id: ApplicationUser_Id,
    verification_message_sent: bool,
    application_user_authorization_token_can_be_resent_from: ApplicationUserAuthorizationToken_CanBeResentFrom,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_WrongEmailOrNicknameOrPassword,
    }
);

enum ApplicationUser_Aggregator {
    First {
        application_user: ApplicationUser1,
    },
    Second {
        application_user: ApplicationUser2,
        application_user_email: ApplicationUser_Email,
    },
}

enum ApplicationUserAuthorizationToken_Aggregator<'a> {
    First {
        application_user_authorization_token: ApplicationUserAuthorizationToken<'a>,
    },
    Second {
        application_user_authorization_token: ApplicationUserAuthorizationToken1,
    },
}
