use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_workflow_event::ApplicationUserWorkflowEvent;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
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

        if ApplicationUserValidator::is_valid_nickname(application_user_nickname.as_str()) {
            match postgresql_connection_pool.get().await {
                Ok(mut pooled_connection) => {
                    match ApplicationUserDataProviderPostgresql::is_exist_by_nickanme(
                        &mut *pooled_connection, application_user_nickname.as_str()
                    ).await {
                        Ok(result) => {
                            return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(result)));
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
                            ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        }

        return Ok(ActionHandlerResult::new_with_application_user_workflow_event(ApplicationUserWorkflowEvent::InvalidEmail));
    }
}