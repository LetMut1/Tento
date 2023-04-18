use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_1;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_2;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_3;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser_1;
use crate::domain_layer::data::entity::application_user::ApplicationUser_2;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::domain_layer::functionality::service::expiration_time_resolver::ExpirationTimeResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::sending_opportunity_resolver::SendingOpportunityResolver;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::borrow::Cow;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(      // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ArgumentResult<ActionProcessorResult<Outcoming>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if !Validator::<ApplicationUser_Password>::is_valid(incoming.application_user_password.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Password });
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(incoming.application_user_device_id.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserDevice_Id });
        }

        let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(
            incoming.application_user_email_or_application_user_nickname.as_str()
        ) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        let application_user_aggregator = if is_valid_email {
            let application_user_ = match PostgresqlRepository::<ApplicationUser_2>::find_2(
                database_1_postgresql_connection,
                incoming.application_user_email_or_application_user_nickname.as_str()
            ).await {
                Ok(application_user__) => application_user__,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            let application_user__ = match application_user_ {
                Some(application_user___) => application_user___,
                None => {
                    return Ok(
                        ArgumentResult::Ok {
                            subject: ActionProcessorResult::UserWorkflowPrecedent {
                                user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser_NotFound
                            }
                        }
                    );
                }
            };

            ApplicationUser_Aggregator::Second { application_user: application_user__ }
        } else {
            if Validator::<ApplicationUser_Nickname>::is_valid(incoming.application_user_email_or_application_user_nickname.as_str()) {
                let application_user_ = match PostgresqlRepository::<ApplicationUser_1>::find_1(
                    database_1_postgresql_connection,
                    incoming.application_user_email_or_application_user_nickname.as_str()
                ).await {
                    Ok(application_user__) => application_user__,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let application_user__ = match application_user_ {
                    Some(application_user___) => application_user___,
                    None => {
                        return Ok(
                            ArgumentResult::Ok {
                                subject: ActionProcessorResult::UserWorkflowPrecedent {
                                    user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser_NotFound
                                }
                            }
                        );
                    }
                };

                ApplicationUser_Aggregator::First { application_user: application_user__ }
            } else {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Nickname });
            }
        };

        let application_user_password_hash = match application_user_aggregator {
            ApplicationUser_Aggregator::First { ref application_user } => application_user.get_password_hash(),
            ApplicationUser_Aggregator::Second { ref application_user } => application_user.get_password_hash()
        };

        let is_valid = match Encoder::<ApplicationUser_Password>::is_valid(
            incoming.application_user_password.as_str(),
            application_user_password_hash
        ) {
            Ok(is_valid_) => is_valid_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if !is_valid {
            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser_WrongPassword
                    }
                }
            );
        }

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_id = match application_user_aggregator {
            ApplicationUser_Aggregator::First { ref application_user } => application_user.get_id(),
            ApplicationUser_Aggregator::Second { ref application_user } => application_user.get_id()
        };

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_authorization_token = match PostgresqlRepository::<ApplicationUserAuthorizationToken_1>::find_1(
            database_2_postgresql_connection,
            application_user_id,
            incoming.application_user_device_id.as_str()
        ).await {
            Ok(application_user_authorization_token_) => application_user_authorization_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let (application_user_authorization_token_aggregator, can_send) = match application_user_authorization_token {
            Some(mut application_user_authorization_token_) => {
                let (can_send_, need_to_update_1) = if SendingOpportunityResolver::<ApplicationUserAuthorizationToken<'_>>::can_send(
                    &application_user_authorization_token_
                ) {
                    let application_user_authorization_token_can_be_resent_from = match Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate() {
                        Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_authorization_token_.set_can_be_resent_from(application_user_authorization_token_can_be_resent_from);

                    (true, true)
                } else {
                    (false, false)
                };

                let need_to_update_2 = if ExpirationTimeResolver::<ApplicationUserAuthorizationToken<'_>>::is_expired(&application_user_authorization_token_) {
                    let application_user_authorization_token_expires_at = match Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate() {
                        Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_authorization_token_
                        .set_value(Generator::<ApplicationUserAuthorizationToken_Value>::generate())
                        .set_wrong_enter_tries_quantity(0)
                        .set_expires_at(application_user_authorization_token_expires_at);

                    true
                } else {
                    false
                };

                if need_to_update_1 && need_to_update_2 {
                    if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken_1>::update(
                        database_2_postgresql_connection,
                        &application_user_authorization_token_,
                        application_user_id,
                        incoming.application_user_device_id.as_str()
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                } else {
                    if need_to_update_1 {
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken_3>::update(
                            database_2_postgresql_connection,
                            &application_user_authorization_token_,
                            application_user_id,
                            incoming.application_user_device_id.as_str()
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }

                    if need_to_update_2 {
                        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAuthorizationToken_2>::update(
                            database_2_postgresql_connection,
                            &application_user_authorization_token_,
                            application_user_id,
                            incoming.application_user_device_id.as_str()
                        ).await {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }
                }

                (
                    ApplicationUserAuthorizationToken_Aggregator::Second {
                        application_user_authorization_token: application_user_authorization_token_
                    },
                    can_send_
                )
            }
            None => {
                let application_user_authorization_token_expires_at = match Generator::<ApplicationUserAuthorizationToken_ExpiresAt>::generate() {
                    Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let application_user_authorization_token_can_be_resent_from = match Generator::<ApplicationUserAuthorizationToken_CanBeResentFrom>::generate() {
                    Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let insert = Insert {
                    application_user_id,
                    application_user_device_id: Cow::Borrowed(incoming.application_user_device_id.as_str()),
                    application_user_authorization_token_value: Generator::<ApplicationUserAuthorizationToken_Value>::generate(),
                    application_user_authorization_token_wrong_enter_tries_quantity: 0,
                    application_user_authorization_token_expires_at,
                    application_user_authorization_token_can_be_resent_from
                };

                let application_user_authorization_token_ = match PostgresqlRepository::<ApplicationUserAuthorizationToken<'_>>::create(
                    database_2_postgresql_connection, insert
                ).await {
                    Ok(application_user_authorization_token__) => application_user_authorization_token__,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                (
                    ApplicationUserAuthorizationToken_Aggregator::First {
                        application_user_authorization_token: application_user_authorization_token_
                    },
                    true
                )
            }
        };

        let application_user_email = match application_user_aggregator {
            ApplicationUser_Aggregator::First { ref application_user } => application_user.get_email(),
            ApplicationUser_Aggregator::Second { application_user: _ } => incoming.application_user_email_or_application_user_nickname.as_str()
        };

        if can_send {
            let application_user_authorization_token_value = match application_user_authorization_token_aggregator {
                ApplicationUserAuthorizationToken_Aggregator::First { application_user_authorization_token: ref application_user_authorization_token_ } => application_user_authorization_token_.get_value(),
                ApplicationUserAuthorizationToken_Aggregator::Second { application_user_authorization_token: ref application_user_authorization_token_ } => application_user_authorization_token_.get_value()
            };

            if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_authorization_token(
                environment_configuration,
                application_user_authorization_token_value,
                application_user_email,
                incoming.application_user_device_id.as_str()
            ) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }

        let application_user_authorization_token_can_be_resent_from = match application_user_authorization_token_aggregator {
            ApplicationUserAuthorizationToken_Aggregator::First { application_user_authorization_token: ref application_user_authorization_token_ } => application_user_authorization_token_.get_can_be_resent_from(),
            ApplicationUserAuthorizationToken_Aggregator::Second { application_user_authorization_token: ref application_user_authorization_token_ } => application_user_authorization_token_.get_can_be_resent_from()
        };

        let outcoming = Outcoming {
            application_user_id,
            verification_message_sent: can_send,
            application_user_authorization_token_can_be_resent_from
        };

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming } });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_device_id: String,
    application_user_email_or_application_user_nickname: String,
    application_user_password: String
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_id: i64,
    verification_message_sent: bool,
    application_user_authorization_token_can_be_resent_from: i64
}

enum ApplicationUser_Aggregator {
    First {
        application_user: ApplicationUser_1
    },
    Second {
        application_user: ApplicationUser_2
    }
}

enum ApplicationUserAuthorizationToken_Aggregator<'a> {
    First {
        application_user_authorization_token: ApplicationUserAuthorizationToken<'a>
    },
    Second {
        application_user_authorization_token: ApplicationUserAuthorizationToken_1
    }
}