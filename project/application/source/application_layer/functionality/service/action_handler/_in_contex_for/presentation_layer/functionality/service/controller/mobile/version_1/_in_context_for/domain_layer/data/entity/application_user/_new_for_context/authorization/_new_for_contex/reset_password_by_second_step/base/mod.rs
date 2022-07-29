use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_workflow_exception::ApplicationUserResetPasswordTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_second_step::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_second_step::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::domain_layer::functionality::service::validator::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::base::Base as Validator;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserResetPasswordTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserResetPasswordTokenStateManagerPostgresql;
use crate::infrastructure_layer::functionality::service::update_resolver::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::base::Base as UpdateResolver;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    pub async fn handle<T>(
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            application_user_id,
            application_user_reset_password_token_value
        ) = action_handler_incoming_data.into_inner();

        match Validator::is_valid_value(application_user_reset_password_token_value.as_str()) {
            Ok(is_valid_value) => {
                if is_valid_value {
                    match authorization_postgresql_connection_pool.get().await {
                        Ok(authorization_postgresql_pooled_connection) => {
                            let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                            match ApplicationUserResetPasswordTokenDataProviderPostgresql::find_by_application_user_id(
                                authorization_postgresql_connection, application_user_id
                            ).await {
                                Ok(application_user_reset_password_token) => {
                                    if let Some(mut application_user_reset_password_token_) = application_user_reset_password_token {
                                        if !application_user_reset_password_token_.get_is_approved() {
                                            if application_user_reset_password_token_.get_value().as_bytes() == application_user_reset_password_token_value.as_bytes() {
                                                application_user_reset_password_token_.set_is_approved(true);

                                                if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerPostgresql::update(
                                                    authorization_postgresql_connection, &application_user_reset_password_token_, UpdateResolver::new(false, false, true, true)
                                                ).await {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }

                                                return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(true)));
                                            } else {
                                                if let Err(mut error) = WrongEnterTriesQuantityIncrementor::increment(&mut application_user_reset_password_token_) {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }

                                                if application_user_reset_password_token_.get_wrong_enter_tries_quantity() <= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                                    if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerPostgresql::update(
                                                        authorization_postgresql_connection, &application_user_reset_password_token_, UpdateResolver::new(false, true, false, false)
                                                    ).await {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                } else {
                                                    if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerPostgresql::delete(
                                                        authorization_postgresql_connection, application_user_reset_password_token_.get_application_user_id()
                                                    ).await {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                }

                                                return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(false)));
                                            }
                                        }

                                        return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordTokenWorkflowException::AlreadyApproved));
                                    }

                                    return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordTokenWorkflowException::NotFound));
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

                return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordTokenWorkflowException::InvalidValue));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}