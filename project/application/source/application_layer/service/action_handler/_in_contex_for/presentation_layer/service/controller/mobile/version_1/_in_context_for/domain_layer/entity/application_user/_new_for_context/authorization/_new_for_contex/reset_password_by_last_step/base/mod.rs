use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_workflow_event::ApplicationUserResetPasswordTokenWorkflowEvent;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_event::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_workflow_event::ApplicationUserWorkflowEvent;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_last_step::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserResetPasswordTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserResetPasswordTokenStateManagerRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserStateManagerPostgresql;
use crate::infrastructure_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as UpdateResolverApplicationUser;
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
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<()>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    { // TODO  !!!!! Это ресет для пользователя, забывшего пароль. НО также нужно сделать АККУРАТНО ресетпассворд для залогиневшегося пользователя с повторной отправкой старого пароля !!!!!!!!!
        let (
            application_user_id, 
            application_user_password,
            application_user_reset_password_token_value
        ) = action_handler_incoming_data.into_inner();

        if ApplicationUserValidator::is_valid_password(application_user_password.as_str()) {
            match redis_connection_pool.get().await {
                Ok(mut redis_pooled_connection) => {
                    let redis_connection = &mut *redis_pooled_connection;

                    match ApplicationUserResetPasswordTokenDataProviderRedis::find_by_application_user_id(
                        redis_connection, application_user_id
                    ).await {
                        Ok(application_user_reset_password_token) => {
                            if let Some(mut application_user_reset_password_token_) = application_user_reset_password_token {
                                if application_user_reset_password_token_.get_value()== application_user_reset_password_token_value.as_str() {
                                    match postgresql_connection_pool.get().await {
                                        Ok(mut postgresql_pooled_connection) => {
                                            let postgresql_connection = &mut *postgresql_pooled_connection;
        
                                            match ApplicationUserDataProviderPostgresql::find_by_id(postgresql_connection, application_user_id).await {
                                                Ok(application_user) => {
                                                    if let Some(mut application_user_) = application_user {
                                                        match PasswordHashResolver::create(application_user_password.as_str()) {
                                                            Ok(password_hash) => {
                                                                application_user_.set_password_hash(password_hash);
                        
                                                                match UpdateResolverApplicationUser::new(false, false, true) {
                                                                    Ok(update_resolver_application_user) => {
                                                                        if let Err(mut error) = ApplicationUserStateManagerPostgresql::update(postgresql_connection, &application_user_, update_resolver_application_user).await {
                                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                                                            return Err(error);
                                                                        }
                                
                                                                        if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerRedis::delete(redis_connection, &application_user_reset_password_token_).await {
                                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                                                            return Err(error);
                                                                        }
                                        
                                                                        return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(()));
                                                                    }
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                        
                                                                        return Err(error);
                                                                    }
                                                                }
                                                            }
                                                            Err(mut error) => {
                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                                                return Err(error);
                                                            }
                                                        }
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
        
                                if let Err(mut error) = WrongEnterTriesQuantityIncrementor::increment(&mut application_user_reset_password_token_) {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                    
                                    return Err(error);
                                }
        
                                if application_user_reset_password_token_.get_wrong_enter_tries_quantity() <= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                    if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerRedis::create(redis_connection, &application_user_reset_password_token_).await {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                        return Err(error);
                                    }
                                } else {
                                    if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerRedis::delete(redis_connection, &application_user_reset_password_token_).await {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                
                                        return Err(error);
                                    }
                                }
        
                                return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_event(ApplicationUserResetPasswordTokenWorkflowEvent::InvalidValue));
                            }
        
                            return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_event(ApplicationUserResetPasswordTokenWorkflowEvent::NotFound));
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

        return Ok(ActionHandlerResult::new_with_application_user_workflow_event(ApplicationUserWorkflowEvent::InvalidPassword));
    }
}