use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::application_user_access_token_workflow_exception::ApplicationUserAccessTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::_new_for_context::base::base::Base as ActionHandlerOutcomingData;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::json_access_web_token::_new_for_context::extractor::_new_for_context::result::Result as ExtractorResult;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::json_access_web_token::_new_for_context::extractor::Extractor;
use crate::infrastructure_layer::functionality::service::_in_context_for::infrastructure_layer::functionality::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    const LIMIT_MINIMUM_VALUE: i16 = 300;
    const LIMIT_MAXIMUM_VALUE: i16 = 500;

    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            json_access_web_token,
            channel_subscribers_quantity,       // TODO // TODO // TODO // TODO // TODO  не нужно ли проверять на >=0 ?
            order,
            mut limit
        ) = action_handler_incoming_data.into_inner();

        match redis_connection_pool.get().await {
            Ok(mut redis_pooled_connection) => {
                match Extractor::extract(environment_configuration_resolver, json_access_web_token.as_str(), &mut *redis_pooled_connection).await {
                    Ok(result) => {
                        match result {
                            ExtractorResult::ApplicationUserAccessToken { application_user_access_token: _ } => {
                                if !(Self::LIMIT_MINIMUM_VALUE..=Self::LIMIT_MAXIMUM_VALUE).contains(&limit) {
                                    limit = Self::LIMIT_MINIMUM_VALUE;
                                }

                                if !OrderConventionResolver::can_convert(order) {
                                    return Err(
                                        ErrorAuditor::new(
                                            BaseError::InvalidArgumentError,
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }

                                match core_postgresql_connection_pool.get().await {
                                    Ok(core_postgresql_pooled_connection) => {
                                        match ChannelDataProviderPostgresql::per_request_3(
                                            &*core_postgresql_pooled_connection, channel_subscribers_quantity, order, limit
                                        ).await {
                                            Ok(channel_registry) => {
                                                return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(channel_registry)));
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
                            ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                                return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::AlreadyExpired));
                            }
                            ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                                return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::InApplicationUserAccessTokenBlackList));
                            }
                        }
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
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolRedisError { bb8_redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}