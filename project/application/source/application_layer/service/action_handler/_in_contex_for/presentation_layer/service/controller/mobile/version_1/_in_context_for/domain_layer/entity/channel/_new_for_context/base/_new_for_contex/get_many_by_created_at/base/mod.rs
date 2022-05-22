use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::_new_for_context::base::Base as RequestData;
use crate::application_layer::data_transfer_object::response_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::_new_for_context::base::base::Base as ResponseData;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::extractor::Extractor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::service::date_time_resolver::DateTimeResolver;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    const LIMIT: i8 = 30;

    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAuditor>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            json_access_web_token,
            channel_created_at,
            order,
            mut limit
        ) = request_data.into_inner();

        match redis_connection_pool.get().await {
            Ok(mut redis_pooled_connection) => {
                match Extractor::extract(environment_configuration_resolver, json_access_web_token.as_str(), &mut *redis_pooled_connection).await {
                    Ok(_json_access_web_token_) => {
                        if let Some(ref channel_created_at_) = channel_created_at {
                            if !DateTimeResolver::is_valid_timestamp(channel_created_at_.as_str()) {
                                return Err(
                                    ErrorAuditor::new(
                                        ErrorAggregator::InvalidArgumentError,
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                
                        if !OrderConventionResolver::can_convert(order) {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::InvalidArgumentError,
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                
                        if limit <= 0 || limit > Self::LIMIT {
                            limit = Self::LIMIT;
                        }
        
                        match postgresql_connection_pool.get().await {
                            Ok(mut postgresql_pooled_connection) => {
                                match ChannelDataProviderPostgresql::per_request_2(
                                    &mut *postgresql_pooled_connection, &channel_created_at, order, limit as i16
                                ).await {
                                    Ok(channel_registry) => {
                                        return Ok(ResponseData::new(channel_registry));
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