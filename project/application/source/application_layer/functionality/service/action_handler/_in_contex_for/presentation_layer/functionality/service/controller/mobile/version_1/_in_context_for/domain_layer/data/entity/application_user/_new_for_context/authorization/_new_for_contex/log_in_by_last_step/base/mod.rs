use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_exception::ApplicationUserLogInTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::domain_layer::data::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::data::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::domain_layer::functionality::service::factory::_in_context_for::domain_layer::data::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::domain_layer::functionality::service::factory::_in_context_for::domain_layer::data::entity::json_refresh_web_token::_new_for_context::base::Base as JsonRefreshWebTokenFactory;
use crate::domain_layer::functionality::service::validator::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::base::Base as Validator;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserLogInTokenStateManagerPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListStateManagerRedis;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::infrastructure_layer::functionality::service::update_resolver::_in_context_for::domain_layer::data::entity::application_user_log_in_token::_new_for_context::base::Base as UpdateResolver;
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
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,        // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
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

        match Validator::is_valid_value(application_user_log_in_token_value.as_str()) {
            Ok(is_valid_value) => {
                if is_valid_value {
                    match redis_connection_pool.get().await {
                        Ok(mut pooled_connection) => {
                            let redis_connection = &mut *pooled_connection;

                            match authorization_postgresql_connection_pool.get().await {
                                Ok(authorization_postgresql_pooled_connection) => {
                                    let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                                    match ApplicationUserLogInTokenDataProviderPostgresql::find_by_application_user_id_and_device_id(
                                        authorization_postgresql_connection, application_user_id, application_user_log_in_token_device_id.as_str()
                                    ).await {
                                        Ok(application_user_log_in_token) => {
                                            if let Some(mut application_user_log_in_token_) = application_user_log_in_token {
                                                if application_user_log_in_token_.get_value() == application_user_log_in_token_value.as_str() {
                                                    match JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                                                        redis_connection, application_user_log_in_token_.get_application_user_id(), application_user_log_in_token_.get_device_id()
                                                    ).await {
                                                        Ok(json_refresh_web_token) => {
                                                            if let Some(json_refresh_web_token_) = json_refresh_web_token {
                                                                if let Err(mut error) = JsonAccessWebTokenBlackListStateManagerRedis::create(
                                                                    redis_connection, &JsonAccessWebTokenBlackList::new(json_refresh_web_token_.get_json_access_web_token_id())
                                                                ).await {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }

                                                                if let Err(mut error) = RepositoryProxy::delete(redis_connection, &json_refresh_web_token_).await {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }
                                                            }

                                                            let json_refresh_web_token_ = JsonRefreshWebTokenFactory::create_from_id_registry(
                                                                application_user_log_in_token_.get_application_user_id(), application_user_log_in_token_.get_device_id()
                                                            );

                                                            if let Err(mut error) = ApplicationUserLogInTokenStateManagerPostgresql::delete(
                                                                authorization_postgresql_connection, application_user_log_in_token_.get_application_user_id(), application_user_log_in_token_.get_device_id()
                                                            ).await {
                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                return Err(error);
                                                            }

                                                            if let Err(mut error) = RepositoryProxy::create(redis_connection, &json_refresh_web_token_).await {
                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                return Err(error);
                                                            }

                                                            match JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token_) {
                                                                Ok(ref json_access_web_token) => {
                                                                    match SerializationFormResolver::serialize(environment_configuration_resolver, json_access_web_token) {
                                                                        Ok(json_access_web_token_) => {
                                                                            match Encoder::encode(environment_configuration_resolver, &json_refresh_web_token_) {
                                                                                Ok(json_refresh_web_token_) => {
                                                                                    return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(json_access_web_token_, json_refresh_web_token_)));
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

                                                if let Err(mut error) = WrongEnterTriesQuantityIncrementor::increment(&mut application_user_log_in_token_) {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }

                                                if application_user_log_in_token_.get_wrong_enter_tries_quantity() <= ApplicationUserLogInToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                                    if let Err(mut error) = ApplicationUserLogInTokenStateManagerPostgresql::update(
                                                        authorization_postgresql_connection, &application_user_log_in_token_, UpdateResolver::new(false, true, false)
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
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolRedisError { bb8_redis_error: error } } },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }

                return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_exception(ApplicationUserLogInTokenWorkflowException::InvalidValue));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}