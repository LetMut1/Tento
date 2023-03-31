use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_device::Id as ApplicationUserDeviceId;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::functionality::service::application_user_authorization_token__expiration_time_resolver::ApplicationUserAuthorizationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_authorization_token__property_generator::ApplicationUserAuthorizationToken_PropertyGenerator;
use crate::domain_layer::functionality::service::application_user_authorization_token__sending_opportunity_resolver::ApplicationUserAuthorizationToken_SendingOpportunityResolver;
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
    pub async fn process<'a, T>(
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
        if !Validator::<ApplicationUserDeviceId>::is_valid(incoming.application_user_device_id.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserDevice_Id });
        }

        if !Validator::<ApplicationUser_Id>::is_valid(incoming.application_user_id) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Id });
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
            database_2_postgresql_connection, incoming.application_user_id, incoming.application_user_device_id.as_str()
        ).await {
            Ok(application_user_authorization_token_) => application_user_authorization_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let mut application_user_authorization_token_ = match application_user_authorization_token {
            Some(application_user_authorization_token__) => application_user_authorization_token__,
            None => {
                return Ok(
                    ArgumentResult::Ok {
                        subject: ActionProcessorResult::UserWorkflowPrecedent {
                            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAuthorizationToken_NotFound
                        }
                    }
                );
            }
        };

        if ApplicationUserAuthorizationToken_ExpirationTimeResolver::is_expired(&application_user_authorization_token_) {
            if let Err(mut error) = ApplicationUserAuthorizationToken_PostgresqlRepository::delete(
                database_2_postgresql_connection, application_user_authorization_token_.get_application_user_id(), application_user_authorization_token_.get_application_user_device_id()
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAuthorizationToken_AlreadyExpired
                    }
                }
            );
        }

        if !ApplicationUserAuthorizationToken_SendingOpportunityResolver::can_send(&application_user_authorization_token_) {
            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserAuthorizationToken_TimeToResendHasNotCome
                    }
                }
            );
        }

        let application_user_authorization_token_can_be_resent_from = match ApplicationUserAuthorizationToken_PropertyGenerator::generate_can_be_resent_from() {
            Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        application_user_authorization_token_.set_can_be_resent_from(application_user_authorization_token_can_be_resent_from);

        if let Err(mut error) = ApplicationUserAuthorizationToken_PostgresqlRepository::update(
            database_2_postgresql_connection,
            &application_user_authorization_token_
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
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

        let application_user = match ApplicationUser_PostgresqlRepository::find_3(
            &*database_1_postgresql_pooled_connection, application_user_authorization_token_.get_application_user_id()
        ).await {
            Ok(application_user_) => application_user_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
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

        if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_authorization_token(
            environment_configuration,
            application_user_authorization_token_.get_value(),
            application_user_.get_email(),
            application_user_authorization_token_.get_application_user_device_id()
        ) {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        let outcoming = Outcoming {
            application_user_authorization_token_can_be_resent_from: application_user_authorization_token_.get_can_be_resent_from()
        };

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Outcoming { outcoming } });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_device_id: String,
    application_user_id: i64
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_authorization_token_can_be_resent_from: i64
}