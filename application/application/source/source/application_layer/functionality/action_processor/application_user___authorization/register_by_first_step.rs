use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken1;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken2;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken3;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_IsApproved;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
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
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By5;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::insert::Insert5;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update7;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update8;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update9;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_first_step::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_first_step::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_first_step::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RegisterByFirstStep;

impl ActionProcessor<ApplicationUser__Authorization___RegisterByFirstStep> {
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

        let is_exist_2 = match PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
            &*database_1_postgresql_pooled_connection,
            &By2 {
                application_user_email: &incoming_.application_user_email,
            },
        )
        .await
        {
            Ok(is_exist_2_) => is_exist_2_,
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

        if is_exist_2 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUser_EmailAlreadyExist),
                },
            );
        }

        let by_5 = By5 {
            application_user_email: &incoming_.application_user_email,
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

        let application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken1>::find_1(
            database_2_postgresql_connection,
            &by_5,
        )
        .await
        {
            Ok(application_user_registration_token_) => application_user_registration_token_,
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

        let (application_user_registration_token_value, application_user_registration_token_can_be_resent_from, application_user_registration_token_wrong_enter_tries_quantity, can_send) = match application_user_registration_token {
            Some(mut application_user_registration_token_) => {
                let (can_send_, need_to_update_1) = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token_.can_be_resent_from.0) {
                    application_user_registration_token_.can_be_resent_from = match Generator::<ApplicationUserRegistrationToken_CanBeResentFrom>::generate() {
                        Ok(application_user_registration_token_can_be_resent_from) => application_user_registration_token_can_be_resent_from,
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

                let need_to_update_2 = if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token_.expires_at.0) || application_user_registration_token_.is_approved.0 {
                    application_user_registration_token_.value = Generator::<ApplicationUserRegistrationToken_Value>::generate();

                    application_user_registration_token_.wrong_enter_tries_quantity = ApplicationUserRegistrationToken_WrongEnterTriesQuantity(0);

                    application_user_registration_token_.is_approved = ApplicationUserRegistrationToken_IsApproved(false);

                    application_user_registration_token_.expires_at = match Generator::<ApplicationUserRegistrationToken_ExpiresAt>::generate() {
                        Ok(application_user_registration_token_expires_at) => application_user_registration_token_expires_at,
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
                    if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken1>::update(
                        database_2_postgresql_connection,
                        &Update7 {
                            application_user_registration_token_value: &application_user_registration_token_.value,
                            application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token_.wrong_enter_tries_quantity,
                            application_user_registration_token_is_approved: application_user_registration_token_.is_approved,
                            application_user_registration_token_expires_at: application_user_registration_token_.expires_at,
                            application_user_registration_token_can_be_resent_from: application_user_registration_token_.can_be_resent_from,
                        },
                        &by_5,
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
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken2>::update(
                            database_2_postgresql_connection,
                            &Update8 {
                                application_user_registration_token_can_be_resent_from: application_user_registration_token_.can_be_resent_from,
                            },
                            &by_5,
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
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken3>::update(
                            database_2_postgresql_connection,
                            &Update9 {
                                application_user_registration_token_value: &application_user_registration_token_.value,
                                application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token_.wrong_enter_tries_quantity,
                                application_user_registration_token_is_approved: application_user_registration_token_.is_approved,
                                application_user_registration_token_expires_at: application_user_registration_token_.expires_at,
                            },
                            &by_5,
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
                    application_user_registration_token_.value,
                    application_user_registration_token_.can_be_resent_from,
                    application_user_registration_token_.wrong_enter_tries_quantity,
                    can_send_,
                )
            }
            None => {
                let application_user_registration_token_expires_at = match Generator::<ApplicationUserRegistrationToken_ExpiresAt>::generate() {
                    Ok(application_user_registration_token_expires_at_) => application_user_registration_token_expires_at_,
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

                let application_user_registration_token_can_be_resent_from = match Generator::<ApplicationUserRegistrationToken_CanBeResentFrom>::generate() {
                    Ok(application_user_registration_token_can_be_resent_from_) => application_user_registration_token_can_be_resent_from_,
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

                let application_user_registration_token_ = match PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::create(
                    database_2_postgresql_connection,
                    Insert5 {
                        application_user_email: &incoming_.application_user_email,
                        application_user_device_id: &incoming_.application_user_device_id,
                        application_user_registration_token_value: Generator::<ApplicationUserRegistrationToken_Value>::generate(),
                        application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity(0),
                        application_user_registration_token_is_approved: ApplicationUserRegistrationToken_IsApproved(false),
                        application_user_registration_token_expires_at,
                        application_user_registration_token_can_be_resent_from,
                    },
                )
                .await
                {
                    Ok(application_user_registration_token___) => application_user_registration_token___,
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
                    application_user_registration_token_.value,
                    application_user_registration_token_.can_be_resent_from,
                    application_user_registration_token_.wrong_enter_tries_quantity,
                    true,
                )
            }
        };

        if can_send {
            if let Err(mut error) = EmailSender::<ApplicationUserRegistrationToken<'_>>::send(
                &application_user_registration_token_value,
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
            verification_message_sent: can_send,
            application_user_registration_token_can_be_resent_from,
            application_user_registration_token_wrong_enter_tries_quantity,
            application_user_registration_token_wrong_enter_tries_quantity_limit: ApplicationUserRegistrationToken_WrongEnterTriesQuantity::LIMIT,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_filled(outcoming),
            },
        );
    }
}
