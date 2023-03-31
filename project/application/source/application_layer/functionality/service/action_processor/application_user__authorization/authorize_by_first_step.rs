use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::data::entity::application_user_authorization_token::Value as ApplicationUserAuthorizationTokenValue;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice;
use crate::domain_layer::data::entity::application_user_device::Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::Email;
use crate::domain_layer::data::entity::application_user::Nickname;
use crate::domain_layer::data::entity::application_user::Password;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user_authorization_token__expiration_time_resolver::ApplicationUserAuthorizationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_authorization_token__property_generator::ApplicationUserAuthorizationToken_PropertyGenerator;
use crate::domain_layer::functionality::service::application_user_authorization_token__sending_opportunity_resolver::ApplicationUserAuthorizationToken_SendingOpportunityResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::ApplicationUserAuthorizationToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
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
        if !Validator::<ApplicationUser<'_>, Password>::is_valid(incoming.application_user_password.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Password });
        }

        if !Validator::<ApplicationUserDevice, Id>::is_valid(incoming.application_user_device_id.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserDevice_Id });
        }

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

        let is_valid_email = match Validator::<ApplicationUser<'_>, Email>::is_valid(incoming.application_user_email_or_application_user_nickname.as_str()) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let application_user = if is_valid_email {
            let application_user_ = match ApplicationUser_PostgresqlRepository::find_2(
                database_1_postgresql_connection, incoming.application_user_email_or_application_user_nickname.as_str()
            ).await {
                Ok(application_user__) => application_user__,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            application_user_
        } else {
            if Validator::<ApplicationUser<'_>, Nickname>::is_valid(incoming.application_user_email_or_application_user_nickname.as_str()) {
                let application_user_ = match ApplicationUser_PostgresqlRepository::find_1(
                    database_1_postgresql_connection, incoming.application_user_email_or_application_user_nickname.as_str()
                ).await {
                    Ok(application_user__) => application_user__,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                application_user_
            } else {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Nickname });
            }
        };

        let application_user_ = match application_user {
            Some(application_user__) => application_user__,
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

        let is_valid = match ApplicationUser_PasswordHashResolver::is_valid(incoming.application_user_password.as_str(), application_user_.get_password_hash()) {
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
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_authorization_token = match ApplicationUserAuthorizationToken_PostgresqlRepository::find_1(
            database_2_postgresql_connection, application_user_.get_id(), incoming.application_user_device_id.as_str()
        ).await {
            Ok(application_user_authorization_token_) => application_user_authorization_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let (application_user_authorization_token_, can_send) = match application_user_authorization_token {
            Some(mut application_user_authorization_token__) => {
                let (can_send_, mut need_to_update) = if ApplicationUserAuthorizationToken_SendingOpportunityResolver::can_send(
                    &application_user_authorization_token__
                ) {
                    let application_user_authorization_token_can_be_resent_from = match ApplicationUserAuthorizationToken_PropertyGenerator::generate_can_be_resent_from() {
                        Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_authorization_token__.set_can_be_resent_from(application_user_authorization_token_can_be_resent_from);

                    (true, true)
                } else {
                    (false, false)
                };

                if ApplicationUserAuthorizationToken_ExpirationTimeResolver::is_expired(&application_user_authorization_token__) {
                    need_to_update = true;

                    let application_user_authorization_token_expires_at = match ApplicationUserAuthorizationToken_PropertyGenerator::generate_expires_at() {
                        Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    application_user_authorization_token__
                        .set_value(Generator::<ApplicationUserAuthorizationToken<'_>, ApplicationUserAuthorizationTokenValue>::generate())
                        .set_wrong_enter_tries_quantity(0)
                        .set_expires_at(application_user_authorization_token_expires_at);
                }

                if need_to_update {
                    if let Err(mut error) = ApplicationUserAuthorizationToken_PostgresqlRepository::update(
                        database_2_postgresql_connection,
                        &application_user_authorization_token__
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                }

                (application_user_authorization_token__, can_send_)
            }
            None => {
                let application_user_authorization_token_expires_at = match ApplicationUserAuthorizationToken_PropertyGenerator::generate_expires_at() {
                    Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let application_user_authorization_token_can_be_resent_from = match ApplicationUserAuthorizationToken_PropertyGenerator::generate_can_be_resent_from() {
                    Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                let insert = Insert {
                    application_user_id: application_user_.get_id(),
                    application_user_device_id: Cow::Borrowed(incoming.application_user_device_id.as_str()),
                    application_user_authorization_token_value: Generator::<ApplicationUserAuthorizationToken<'_>, ApplicationUserAuthorizationTokenValue>::generate(),
                    application_user_authorization_token_wrong_enter_tries_quantity: 0,
                    application_user_authorization_token_expires_at,
                    application_user_authorization_token_can_be_resent_from
                };

                let application_user_authorization_token__ = match ApplicationUserAuthorizationToken_PostgresqlRepository::create(
                    database_2_postgresql_connection, insert
                ).await {
                    Ok(application_user_authorization_token___) => application_user_authorization_token___,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                (application_user_authorization_token__, true)
            }
        };

        if can_send {
            if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_authorization_token(
                environment_configuration,
                application_user_authorization_token_.get_value(),
                application_user_.get_email(),
                application_user_authorization_token_.get_application_user_device_id()
            ) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }

        let outcoming = Outcoming {
            application_user_id: application_user_.get_id(),
            verification_message_sent: can_send,
            application_user_authorization_token_can_be_resent_from: application_user_authorization_token_.get_application_user_id()
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