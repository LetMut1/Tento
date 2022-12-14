use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::value_generator::ValueGenerator;
use crate::domain_layer::functionality::service::validator::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::base::Base as Validator;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::_new_for_context::insert::Insert;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserResetPasswordTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserResetPasswordTokenStateManagerPostgresql;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use serde::Deserialize;
use serde::Serialize;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
pub struct ActionHandlerIncomingData {
    application_user_email: String
}

impl ActionHandlerIncomingData {
    pub fn into_inner(
        self
    ) -> String {
        return self.application_user_email;
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
pub struct ActionHandlerOutcomingData {
    application_user_id: i64
}

impl ActionHandlerOutcomingData {
    pub fn new(
        application_user_id: i64
    ) -> Self {
        return Self {
            application_user_id
        };
    }
}

pub struct Base;

impl Base {
    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let application_user_email = action_handler_incoming_data.into_inner();

        match Validator::is_valid_email(application_user_email.as_str()) {
            Ok(is_valid_email) => {
                if is_valid_email {
                    match core_postgresql_connection_pool.get().await {
                        Ok(core_postgresql_pooled_connection) => {
                            match ApplicationUserDataProviderPostgresql::find_2(
                                &*core_postgresql_pooled_connection, application_user_email
                            ).await {
                                Ok(application_user) => {
                                    if let Some(application_user_) = application_user {
                                        let application_user_id = application_user_.get_id();

                                        match authorization_postgresql_connection_pool.get().await {
                                            Ok(authorization_postgresql_pooled_connection) => {
                                                let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                                                match ApplicationUserResetPasswordTokenDataProviderPostgresql::find_1(
                                                    authorization_postgresql_connection, application_user_id
                                                ).await {
                                                    Ok(application_user_reset_password_token_) => {
                                                        let application_user_reset_password_token = match application_user_reset_password_token_ {
                                                            Some(mut application_user_reset_password_token__) => {
                                                                let expires_at = match DateTimeResolver::add_interval_from_now_formated(ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
                                                                    Ok(expires_at_) => expires_at_,
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                        return Err(error);
                                                                    }
                                                                };

                                                                let is_expired = match ExpirationTimeResolver::is_expired(&application_user_reset_password_token__) {
                                                                    Ok(is_expired_) => is_expired_,
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                        return Err(error);
                                                                    }
                                                                };

                                                                application_user_reset_password_token__.set_expires_at(expires_at);
                                                                if is_expired || application_user_reset_password_token__.get_is_approved() {
                                                                    application_user_reset_password_token__
                                                                        .set_value(ValueGenerator::generate())
                                                                        .set_wrong_enter_tries_quantity(0)
                                                                        .set_is_approved(false);
                                                                }

                                                                if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerPostgresql::update(
                                                                    authorization_postgresql_connection, &application_user_reset_password_token__
                                                                ).await {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }

                                                                application_user_reset_password_token__
                                                            }
                                                            None => {
                                                                let insert = Insert::new(
                                                                    application_user_id,
                                                                    ValueGenerator::generate(),
                                                                    0,
                                                                    false
                                                                );

                                                                match ApplicationUserResetPasswordTokenStateManagerPostgresql::create(
                                                                    authorization_postgresql_connection, insert
                                                                ).await {
                                                                    Ok(application_user_reset_password_token__) => application_user_reset_password_token__,
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                        return Err(error);
                                                                    }
                                                                }
                                                            }
                                                        };

                                                        if let Err(mut error) = EmailSender::send_application_user_reset_password_token(
                                                            environment_configuration_resolver, application_user_reset_password_token.get_value(), application_user_.get_email()
                                                        ) {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                            return Err(error);
                                                        }

                                                        return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(application_user_id)));
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

                return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidEmail));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}