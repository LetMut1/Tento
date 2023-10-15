use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser3;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken1;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken2;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken3;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_IsApproved;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use crate::domain_layer::functionality::service::email_sender::EmailSender;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By2;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert6;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update12;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update13;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update14;
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
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Precedent;

pub struct ResetPasswordByFirstStep;

impl ResetPasswordByFirstStep {
    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Option<Incoming>,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor_>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = match incoming {
            Some(incoming__) => incoming__,
            None => {
                return Err(
                    ErrorAuditor_::new(
                        Error::create_incoming_invalid_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(&incoming_.application_user_email) {
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

        if !is_valid_email {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Email,
                },
            );
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(&incoming_.application_user_device_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserDevice_Id,
                },
            );
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
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

        let application_user = match PostgresqlRepository::<ApplicationUser3>::find_1(
            &*database_1_postgresql_pooled_connection,
            &By2 {
                application_user_email: &incoming_.application_user_email,
            },
        )
        .await
        {
            Ok(application_user_) => application_user_,
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

        let application_user_ = match application_user {
            Some(application_user__) => application_user__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUser_NotFound),
                    },
                );
            }
        };

        let by_4 = By4 {
            application_user_id: application_user_.id,
            application_user_device_id: &incoming_.application_user_device_id,
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
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

        let application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken1>::find_1(
            database_2_postgresql_connection,
            &by_4,
        )
        .await
        {
            Ok(application_user_reset_password_token_) => application_user_reset_password_token_,
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

        let (application_user_reset_password_token_value, application_user_reset_password_token_can_be_resent_from, application_user_reset_password_token_wrong_enter_tries_quantity, can_send) = match application_user_reset_password_token {
            Some(mut application_user_reset_password_token_) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token_.can_be_resent_from.0) {
                    application_user_reset_password_token_.can_be_resent_from = match Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate() {
                        Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
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

                let need_to_update_2 = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token_.expires_at.0) || application_user_reset_password_token_.is_approved.0 {
                    application_user_reset_password_token_.value = Generator::<ApplicationUserResetPasswordToken_Value>::generate();

                    application_user_reset_password_token_.wrong_enter_tries_quantity = ApplicationUserResetPasswordToken_WrongEnterTriesQuantity(0);

                    application_user_reset_password_token_.is_approved = ApplicationUserResetPasswordToken_IsApproved(false);

                    application_user_reset_password_token_.expires_at = match Generator::<ApplicationUserResetPasswordToken_ExpiresAt>::generate() {
                        Ok(application_user_reset_password_token_expires_at) => application_user_reset_password_token_expires_at,
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
                    if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken1>::update(
                        database_2_postgresql_connection,
                        &Update12 {
                            application_user_reset_password_token_value: &application_user_reset_password_token_.value,
                            application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token_.wrong_enter_tries_quantity,
                            application_user_reset_password_token_is_approved: application_user_reset_password_token_.is_approved,
                            application_user_reset_password_token_expires_at: application_user_reset_password_token_.expires_at,
                            application_user_reset_password_token_can_be_resent_from: application_user_reset_password_token_.can_be_resent_from,
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
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken2>::update(
                            database_2_postgresql_connection,
                            &Update13 {
                                application_user_reset_password_token_can_be_resent_from: application_user_reset_password_token_.can_be_resent_from,
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
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken3>::update(
                            database_2_postgresql_connection,
                            &Update14 {
                                application_user_reset_password_token_value: &application_user_reset_password_token_.value,
                                application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token_.wrong_enter_tries_quantity,
                                application_user_reset_password_token_is_approved: application_user_reset_password_token_.is_approved,
                                application_user_reset_password_token_expires_at: application_user_reset_password_token_.expires_at,
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
                    application_user_reset_password_token_.value,
                    application_user_reset_password_token_.can_be_resent_from,
                    application_user_reset_password_token_.wrong_enter_tries_quantity,
                    can_send_,
                )
            }
            None => {
                let application_user_reset_password_token_expires_at = match Generator::<ApplicationUserResetPasswordToken_ExpiresAt>::generate() {
                    Ok(application_user_reset_password_token_expires_at) => application_user_reset_password_token_expires_at,
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

                let application_user_reset_password_token_can_be_resent_from = match Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate() {
                    Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
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

                let application_user_reset_password_token_ = match PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::create(
                    database_2_postgresql_connection,
                    Insert6 {
                        application_user_id: application_user_.id,
                        application_user_device_id: &incoming_.application_user_device_id,
                        application_user_reset_password_token_value: Generator::<ApplicationUserResetPasswordToken_Value>::generate(),
                        application_user_reset_password_token_wrong_enter_tries_quantity: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity(0),
                        application_user_reset_password_token_is_approved: ApplicationUserResetPasswordToken_IsApproved(false),
                        application_user_reset_password_token_expires_at,
                        application_user_reset_password_token_can_be_resent_from,
                    },
                )
                .await
                {
                    Ok(application_user_reset_password_token___) => application_user_reset_password_token___,
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
                    application_user_reset_password_token_.value,
                    application_user_reset_password_token_.can_be_resent_from,
                    application_user_reset_password_token_.wrong_enter_tries_quantity,
                    true,
                )
            }
        };

        if can_send {
            if let Err(mut error) = EmailSender::<ApplicationUserResetPasswordToken<'_>>::send(
                &application_user_reset_password_token_value,
                &incoming_.application_user_email,
                &incoming_.application_user_device_id,
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
            application_user_id: application_user_.id,
            verification_message_sent: can_send,
            application_user_reset_password_token_can_be_resent_from,
            application_user_reset_password_token_wrong_enter_tries_quantity,
            application_user_reset_password_token_wrong_enter_tries_quantity_limit: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::LIMIT,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_filled(outcoming),
            },
        );
    }
}
