use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_exception::ApplicationUserLogInTokenWorkflowException;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use serde::Deserialize;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Serialize;

pub struct Base;

impl Base {
    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<()>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    { // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_log_in_token_device_id,
            application_user_id
        ) = action_handler_incoming_data.into_inner();

        match authorization_postgresql_connection_pool.get().await {
            Ok(authorization_postgresql_pooled_connection) => {
                match ApplicationUserLogInTokenDataProviderPostgresql::find_1(
                    &*authorization_postgresql_pooled_connection, application_user_id, application_user_log_in_token_device_id.as_str()
                ).await {
                    Ok(application_user_log_in_token) => {
                        if let Some(application_user_log_in_token_) = application_user_log_in_token {
                            match core_postgresql_connection_pool.get().await {
                                Ok(core_postgresql_pooled_connection) => {
                                    match ApplicationUserDataProviderPostgresql::find_3(
                                        &*core_postgresql_pooled_connection, application_user_id
                                    ).await {
                                        Ok(application_user) => {
                                            if let Some(application_user_) = application_user {
                                                if let Err(mut error) = EmailSender::send_application_user_log_in_token(
                                                   environment_configuration_resolver, application_user_log_in_token_.get_value(), application_user_.get_email()
                                                ) {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }

                                                return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(()));
                                            }

                                            return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::NotFound));
                                        }
                                        Err(mut error) => {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                            return Err(error);
                                        }
                                    }
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }
                        }

                        return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_exception(ApplicationUserLogInTokenWorkflowException::NotFound));
                    }
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
pub struct ActionHandlerIncomingData {
    application_user_log_in_token_device_id: String,
    application_user_id: i64
}

impl ActionHandlerIncomingData {
    pub fn into_inner(
        self
    ) -> (String, i64) {
        return (
            self.application_user_log_in_token_device_id,
            self.application_user_id
        );
    }
}