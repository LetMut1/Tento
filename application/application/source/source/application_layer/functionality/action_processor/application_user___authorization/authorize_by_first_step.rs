use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser1;
use crate::domain_layer::data::entity::application_user::ApplicationUser2;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
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
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By2;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert3;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update3;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update5;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Precedent;

pub struct AuthorizeByFirstStep;

impl AuthorizeByFirstStep {
    pub async fn process<'a, T>(
        // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
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
        if !Validator::<ApplicationUser_Password>::is_valid_part_1(&incoming.application_user_password) {
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

        let application_user_email = ApplicationUser_Email(incoming.application_user_email_or_application_user_nickname);

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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::ConnectionPoolPostgresql {
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

        let (application_user_id, application_user_email, application_user_nickname, application_user_password_hash) = if is_valid_email {
            let application_user_ = match PostgresqlRepository::<ApplicationUser2>::find_1(
                database_1_postgresql_connection,
                &By2 {
                    application_user_email: &application_user_email,
                },
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

            (
                application_user__.id,
                application_user_email,
                application_user__.nickname,
                application_user__.password_hash,
            )
        } else {
            let application_user_nickname = ApplicationUser_Nickname(application_user_email.0);

            if Validator::<ApplicationUser_Nickname>::is_valid(&application_user_nickname) {
                let application_user_ = match PostgresqlRepository::<ApplicationUser1>::find_1(
                    database_1_postgresql_connection,
                    &By1 {
                        application_user_nickname: &application_user_nickname,
                    },
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

                (
                    application_user__.id,
                    application_user__.email,
                    application_user_nickname,
                    application_user__.password_hash,
                )
            } else {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument: InvalidArgument::ApplicationUser_Nickname,
                    },
                );
            }
        };

        if !Validator::<ApplicationUser_Password>::is_valid_part_2(
            &incoming.application_user_password,
            &application_user_email,
            &application_user_nickname,
        ) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Password,
                },
            );
        }

        let is_valid = match Encoder::<ApplicationUser_Password>::is_valid(
            &incoming.application_user_password,
            &application_user_password_hash,
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

        let by_4 = By4 {
            application_user_id,
            application_user_device_id: &incoming.application_user_device_id,
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::ConnectionPoolPostgresql {
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

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_authorization_token = match PostgresqlRepository::<ApplicationUserAuthorizationToken1>::find_1(
            database_2_postgresql_connection,
            &by_4,
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

        let (application_user_authorization_token_value, application_user_authorization_token_can_be_resent_from, application_user_authorization_token_wrong_enter_tries_quantity, can_send) = match application_user_authorization_token {
            Some(mut application_user_authorization_token_) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token_.can_be_resent_from.0) {
                    application_user_authorization_token_.can_be_resent_from = match Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate() {
                        Ok(application_user_authorization_token_can_be_resent_from) => application_user_authorization_token_can_be_resent_from,
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
                        true, true,
                    )
                } else {
                    (
                        false, false,
                    )
                };

                let need_to_update_2 = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_authorization_token_.expires_at.0) {
                    application_user_authorization_token_.value = Generator::<ApplicationUserAuthorizationToken_Value>::generate();

                    application_user_authorization_token_.wrong_enter_tries_quantity = ApplicationUserAuthorizationToken_WrongEnterTriesQuantity(0);

                    application_user_authorization_token_.expires_at = match Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate() {
                        Ok(application_user_authorization_token_expires_at) => application_user_authorization_token_expires_at,
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

                    true
                } else {
                    false
                };

                if need_to_update_1 && need_to_update_2 {
                    if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken1>::update(
                        database_2_postgresql_connection,
                        &Update3 {
                            application_user_authorization_token_value: &application_user_authorization_token_.value,
                            application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                            application_user_authorization_token_expires_at: application_user_authorization_token_.expires_at,
                            application_user_authorization_token_can_be_resent_from: application_user_authorization_token_.can_be_resent_from,
                        },
                        &by_4,
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
                            &Update5 {
                                application_user_authorization_token_can_be_resent_from: application_user_authorization_token_.can_be_resent_from,
                            },
                            &by_4,
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
                            &Update4 {
                                application_user_authorization_token_value: &application_user_authorization_token_.value,
                                application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token_.wrong_enter_tries_quantity,
                                application_user_authorization_token_expires_at: application_user_authorization_token_.expires_at,
                            },
                            &by_4,
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
                    application_user_authorization_token_.value,
                    application_user_authorization_token_.can_be_resent_from,
                    application_user_authorization_token_.wrong_enter_tries_quantity,
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

                let application_user_authorization_token_ = match PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::create(
                    database_2_postgresql_connection,
                    Insert3 {
                        application_user_id,
                        application_user_device_id: &incoming.application_user_device_id,
                        application_user_authorization_token_value: Generator::<ApplicationUserAuthorizationToken_Value>::generate(),
                        application_user_authorization_token_wrong_enter_tries_quantity: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity(0),
                        application_user_authorization_token_expires_at,
                        application_user_authorization_token_can_be_resent_from,
                    },
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
                    application_user_authorization_token_.value,
                    application_user_authorization_token_.can_be_resent_from,
                    application_user_authorization_token_.wrong_enter_tries_quantity,
                    true,
                )
            }
        };

        if can_send {
            if let Err(mut error) = EmailSender::<ApplicationUserAuthorizationToken<'_>>::send(
                &application_user_authorization_token_value,
                &application_user_email,
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

        let outcoming = Outcoming {
            application_user_id,
            verification_message_sent: can_send,
            application_user_authorization_token_can_be_resent_from,
            application_user_authorization_token_wrong_enter_tries_quantity,
            application_user_authorization_token_wrong_enter_tries_quantity_limit: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity::LIMIT,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(outcoming),
            },
        );
    }
}