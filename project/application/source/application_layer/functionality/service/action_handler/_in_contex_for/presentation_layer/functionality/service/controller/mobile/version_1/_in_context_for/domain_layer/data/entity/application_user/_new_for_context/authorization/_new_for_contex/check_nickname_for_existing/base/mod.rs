use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::domain_layer::functionality::service::validator::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::base::Base as Validator;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use serde::Deserialize;
use serde::Serialize;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    pub async fn handle<T>(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let application_user_nickname = action_handler_incoming_data.into_inner();

        if Validator::is_valid_nickname(application_user_nickname.as_str()) {
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

            let is_exist = match ApplicationUserDataProviderPostgresql::is_exist_1(&*pooled_connection, application_user_nickname.as_str()).await {
                Ok(is_exist_) => is_exist_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(is_exist)));
        }

        return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidNickname));
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
pub struct ActionHandlerIncomingData {
    application_user_nickname: String
}

impl ActionHandlerIncomingData {
    pub fn into_inner(
        self
    ) -> String {
        return self.application_user_nickname;
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
pub struct ActionHandlerOutcomingData {
    result: bool
}

impl ActionHandlerOutcomingData {
    pub fn new(
        result: bool
    ) -> Self {
        return Self {
            result
        };
    }
}