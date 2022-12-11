use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_exception::ApplicationUserLogInTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::encoder::Encoder;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::obfuscation_value_generator::ObfuscationValueGenerator;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::id_generator::IdGenerator;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::domain_layer::functionality::service::validator::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::base::Base as Validator;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserAccessRefreshTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserAccessRefreshTokenStateManagerPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Insert;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Update;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserLogInTokenStateManagerPostgresql;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use std::borrow::Cow;
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
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,             // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor>   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            application_user_id,
            application_user_log_in_token_device_id,  // TODO ПРоверить все входящие значения application_user_log_in_token_device_id нв формат. Формата может не быть. Нужно определиться, есть ли формат, напримре, UUID
            application_user_log_in_token_value
        ) = action_handler_incoming_data.into_inner();

        let is_valid_value = match Validator::is_valid_value(application_user_log_in_token_value.as_str()) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if is_valid_value {
            let authorization_postgresql_pooled_connection = match authorization_postgresql_connection_pool.get().await {
                Ok(authorization_postgresql_pooled_connection_) => authorization_postgresql_pooled_connection_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };
            let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

            let application_user_log_in_token = match ApplicationUserLogInTokenDataProviderPostgresql::find_1(
                authorization_postgresql_connection, application_user_id, application_user_log_in_token_device_id.as_str()
            ).await {
                Ok(application_user_log_in_token_) => application_user_log_in_token_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };
            let mut application_user_log_in_token_ = match application_user_log_in_token {
                Some(application_user_log_in_token__) => application_user_log_in_token__,
                None => {
                    return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_exception(ApplicationUserLogInTokenWorkflowException::NotFound));
                }
            };

            let is_expired = match ExpirationTimeResolver::is_expired(&application_user_log_in_token_) {
                Ok(is_expired_) => is_expired_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };
            if !is_expired {
                if application_user_log_in_token_.get_value() == application_user_log_in_token_value.as_str() {
                    let expires_at = match DateTimeResolver::add_interval_from_now_formated(ApplicationUserAccessToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
                        Ok(expires_at_) => expires_at_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };
                    let application_user_access_token = ApplicationUserAccessToken::new(
                        IdGenerator::generate(),
                        application_user_log_in_token_.get_application_user_id(),
                        Cow::Borrowed(application_user_log_in_token_.get_device_id()),
                        expires_at
                    );

                    let application_user_access_refresh_token = match ApplicationUserAccessRefreshTokenDataProviderPostgresql::find_1(
                        authorization_postgresql_connection, application_user_log_in_token_.get_application_user_id(), application_user_log_in_token_.get_device_id()
                    ).await {
                        Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };
// TODO  TRANZACTION
                    let application_user_access_refresh_token_ = match application_user_access_refresh_token {
                        Some(mut application_user_access_refresh_token__) => {
                            application_user_access_refresh_token__
                                .set_application_user_access_token_id(Cow::Borrowed(application_user_access_token.get_id()))
                                .set_obfuscation_value(ObfuscationValueGenerator::generate());

                            let update = Update {
                                application_user_access_refresh_token_expires_at: true,
                                application_user_access_refresh_token_updated_at: true
                            };

                            if let Err(mut error) = ApplicationUserAccessRefreshTokenStateManagerPostgresql::update(
                                authorization_postgresql_connection,
                                &mut application_user_access_refresh_token__,
                                update
                            ).await {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                return Err(error);
                            }

                            application_user_access_refresh_token__
                        }
                        None => {
                            let insert = Insert {
                                application_user_id: application_user_log_in_token_.get_application_user_id(),
                                application_user_log_in_token_device_id: Cow::Borrowed(application_user_log_in_token_.get_device_id()),
                                application_user_access_token_id: Cow::Borrowed(application_user_access_token.get_id()),
                                application_user_access_refresh_token_obfuscation_value: ObfuscationValueGenerator::generate(),
                            };

                            match ApplicationUserAccessRefreshTokenStateManagerPostgresql::create(authorization_postgresql_connection, insert).await {
                                Ok(application_user_access_refresh_token___) => application_user_access_refresh_token___,
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                    return Err(error);
                                }
                            }
                        }
                    };

                    if let Err(mut error) = ApplicationUserLogInTokenStateManagerPostgresql::delete(
                        authorization_postgresql_connection, application_user_log_in_token_.get_application_user_id(), application_user_log_in_token_.get_device_id()
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
// TODO  TRANZACTION
                    let application_user_access_token_web_form = match SerializationFormResolver::serialize(environment_configuration_resolver, &application_user_access_token) {
                        Ok(application_user_access_token_web_form_) => application_user_access_token_web_form_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    let application_user_access_refresh_token_web_form = match Encoder::encode(environment_configuration_resolver, &application_user_access_refresh_token_) {
                        Ok(application_user_access_refresh_token_web_form_) => application_user_access_refresh_token_web_form_,
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    return Ok(
                        ActionHandlerResult::new_with_action_handler_outcoming_data(
                            ActionHandlerOutcomingData::new(application_user_access_token_web_form, application_user_access_refresh_token_web_form)
                        )
                    );
                }

                if let Err(mut error) = WrongEnterTriesQuantityIncrementor::increment(&mut application_user_log_in_token_) {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }

                if application_user_log_in_token_.get_wrong_enter_tries_quantity() <= ApplicationUserLogInToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                    if let Err(mut error) = ApplicationUserLogInTokenStateManagerPostgresql::update(
                        authorization_postgresql_connection, &application_user_log_in_token_,
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                } else {
                    if let Err(mut error) = ApplicationUserLogInTokenStateManagerPostgresql::delete(
                        authorization_postgresql_connection, application_user_log_in_token_.get_application_user_id(), application_user_log_in_token_.get_device_id()
                    ).await {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                }

                return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_exception(ApplicationUserLogInTokenWorkflowException::WrongValue));
            }

            return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_exception(ApplicationUserLogInTokenWorkflowException::AlreadyExpired));
        }

        return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_exception(ApplicationUserLogInTokenWorkflowException::InvalidValue));
    }
}