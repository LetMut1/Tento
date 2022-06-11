use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_event::ApplicationUserLogInTokenWorkflowEvent;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_workflow_event::ApplicationUserWorkflowEvent;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
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

        match redis_connection_pool.get().await {
            Ok(mut redis_pooled_connection) => {
                match ApplicationUserLogInTokenDataProviderRedis::find_by_application_user_id_and_device_id(
                    &mut *redis_pooled_connection,
                    application_user_id, application_user_log_in_token_device_id.as_str()
                ).await {
                    Ok(application_user_log_in_token) => {
                        if let Some(application_user_log_in_token_) = application_user_log_in_token {
                            match postgresql_connection_pool.get().await {
                                Ok(mut postgresql_pooled_connection) => {
                                    match ApplicationUserDataProviderPostgresql::find_by_id(
                                        &mut *postgresql_pooled_connection,
                                        application_user_id
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
                                
                                            return Ok(ActionHandlerResult::new_with_application_user_workflow_event(ApplicationUserWorkflowEvent::NotFound));
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
                
                        return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_event(ApplicationUserLogInTokenWorkflowEvent::NotFound));
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
                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolRedisError { bb8_redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}