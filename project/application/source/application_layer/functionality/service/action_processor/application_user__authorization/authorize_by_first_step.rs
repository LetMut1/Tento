use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_authorization_token__expiration_time_resolver::ApplicationUserAuthorizationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_authorization_token__value_generator::ApplicationUserAuthorizationToken_ValueGenerator;
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
use crate::infrastructure_layer::functionality::repository::application_user_authorization_token__postgresql_repository::Update;
use crate::infrastructure_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
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
        if !ApplicationUser_Validator::is_valid_password(incoming.application_user_password.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Password });
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

        let is_valid_email = match ApplicationUser_Validator::is_valid_email(incoming.application_user_email_or_application_user_nickname.as_str()) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let application_user = if is_valid_email {
            let application_user_ = match ApplicationUser_PostgresqlRepository::find_2(
                database_1_postgresql_connection, incoming.application_user_email_or_application_user_nickname
            ).await {
                Ok(application_user__) => application_user__,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            application_user_
        } else {
            if ApplicationUser_Validator::is_valid_nickname(incoming.application_user_email_or_application_user_nickname.as_str()) {
                let application_user_ = match ApplicationUser_PostgresqlRepository::find_1(
                    database_1_postgresql_connection, incoming.application_user_email_or_application_user_nickname
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

        let application_user_id = application_user_.get_id();

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
            database_2_postgresql_connection, application_user_id, incoming.application_user_device_id.as_str()
        ).await {
            Ok(application_user_authorization_token_) => application_user_authorization_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let application_user_authorization_token_ = match application_user_authorization_token {
            Some(mut application_user_authorization_token__) => {
                if ApplicationUserAuthorizationToken_ExpirationTimeResolver::is_expired(&application_user_authorization_token__) {
                    application_user_authorization_token__
                        .set_value(ApplicationUserAuthorizationToken_ValueGenerator::generate())
                        .set_wrong_enter_tries_quantity(0);

                    if let Err(mut error) = ApplicationUserAuthorizationToken_PostgresqlRepository::update(
                        database_2_postgresql_connection,
                        &mut application_user_authorization_token__,
                        Update { application_user_authorization_token_expires_at: true }
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                }

                application_user_authorization_token__
            }
            None => {
                let insert = Insert {
                    application_user_id,
                    application_user_device_id: incoming.application_user_device_id.as_str(),
                    application_user_authorization_token_value: ApplicationUserAuthorizationToken_ValueGenerator::generate(),
                    application_user_authorization_token_wrong_enter_tries_quantity: 0
                };

                match ApplicationUserAuthorizationToken_PostgresqlRepository::create(
                    database_2_postgresql_connection, insert
                ).await {
                    Ok(application_user_authorization_token__) => application_user_authorization_token__,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                }
            }
        };

        if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_authorization_token(
            environment_configuration, application_user_authorization_token_.get_value(), application_user_.get_email()
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming: Outcoming { application_user_id } } });
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
    application_user_id: i64
}