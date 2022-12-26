use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
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
    pub async fn process<T>(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionProcessorResult<Outcoming>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let application_user_email = incoming.into_inner();

        let is_valid_email = match ApplicationUser_Validator::is_valid_email(application_user_email.as_str()) {
            Ok(is_valid_email_) => is_valid_email_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if is_valid_email {
            let pooled_connection = match postgresql_connection_pool.get().await {
                Ok(pooled_connection_) => pooled_connection_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let is_exist = match ApplicationUser_PostgresqlRepository::is_exist_2(&*pooled_connection, application_user_email.as_str()).await {
                Ok(is_exist_) => is_exist_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            return Ok(ActionProcessorResult::new_with_outcoming(Outcoming::new(is_exist)));
        }

        return Ok(ActionProcessorResult::new_with_application_user_workflow_exception(ApplicationUser_WorkflowException::InvalidEmail));
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_email: String
}

impl Incoming {
    pub fn into_inner(
        self
    ) -> String {
        return self.application_user_email;
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    result: bool
}

impl Outcoming {
    pub fn new(
        result: bool
    ) -> Self {
        return Self {
            result
        };
    }
}