use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_first_step::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::value_generator::ValueGenerator;
use crate::domain_layer::functionality::service::validator::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenStateManagerPostgresql;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::infrastructure_layer::functionality::service::update_resolver::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::base::Base as UpdateResolver;
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
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        postgresql_authorization_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<()>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let application_user_email = action_handler_incoming_data.into_inner();
        
        match ApplicationUserValidator::is_valid_email(application_user_email.as_str()) {
            Ok(is_valid_email) => {
                if is_valid_email {
                    match core_postgresql_connection_pool.get().await {
                        Ok(core_postgresql_pooled_connection) => {
                            match ApplicationUserDataProviderPostgresql::is_exist_by_email(
                                &*core_postgresql_pooled_connection, application_user_email.as_str()
                            ).await {
                                Ok(is_exist_by_email) => {
                                    if !is_exist_by_email {
                                        match postgresql_authorization_connection_pool.get().await {
                                            Ok(postgresql_authorization_pooled_connection) => {
                                                let postgresql_authorization_connection = &*postgresql_authorization_pooled_connection;
                
                                                match ApplicationUserRegistrationConfirmationTokenDataProviderPostgresql::find_by_application_user_email(
                                                    postgresql_authorization_connection, application_user_email.as_str()
                                                ).await {
                                                    Ok(application_user_registration_confirmation_token_) => {
                                                        let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_>;

                                                        match application_user_registration_confirmation_token_ {
                                                            Some(application_user_registration_confirmation_token__) => {
                                                                application_user_registration_confirmation_token = application_user_registration_confirmation_token__;
                                        
                                                                match UpdateResolver::new(false, true) {
                                                                    Ok(update_resolver) => {
                                                                        if let Err(mut error) = ApplicationUserRegistrationConfirmationTokenStateManagerPostgresql::update(
                                                                            postgresql_authorization_connection, &application_user_registration_confirmation_token, update_resolver
                                                                        ).await {
                                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                    
                                                                            return Err(error);
                                                                        }
                                                                    }
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                                        
                                                                        return Err(error);
                                                                    }
                                                                }
                                                            }
                                                            None => {
                                                                application_user_registration_confirmation_token = ApplicationUserRegistrationConfirmationToken::new(
                                                                        application_user_email.as_str(),
                                                                        ValueGenerator::generate(),
                                                                        0,
                                                                        None
                                                                    );
                                        
                                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationTokenStateManagerPostgresql::create(
                                                                    postgresql_authorization_connection, &application_user_registration_confirmation_token
                                                                ).await {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                            
                                                                    return Err(error);
                                                                }
                                                            }
                                                        }
                                                        
                                                        if let Err(mut error) = EmailSender::send_application_user_registration_confirmation_token(
                                                            environment_configuration_resolver,
                                                            application_user_registration_confirmation_token.get_value(),
                                                            application_user_email.as_str()
                                                        ) {
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
                                        
                                    return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::EmailAlreadyExist));
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
                
                return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidEmail));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}