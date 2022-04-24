
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::extractor::Extractor;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::base::Base as ResponseData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    const CHANNEL_ID_REGISTRY_LENGTH_LIMIT: usize = 30;

    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAuditor> {
        let (
            json_access_web_token,
            channel_id_registry
        ) = request_data.into_inner();
        
        match redis_connection_pool.get().await {
            Ok(mut redis_pooled_connection) => {
                let _json_access_web_token = Extractor::extract(json_access_web_token.as_str(), &mut *redis_pooled_connection).await?;

                if channel_id_registry.len() == 0 || channel_id_registry.len() > Self::CHANNEL_ID_REGISTRY_LENGTH_LIMIT {
                    return Err(
                        ErrorAuditor::new(
                            ErrorAggregator::InvalidArgumentError,
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
        
                match postgresql_connection_pool.get().await {
                    Ok(mut postgresql_pooled_connection) => {
                        let channel_registry = ChannelDataProviderPostgresql::per_request_4(
                            &mut *postgresql_pooled_connection, &channel_id_registry
                        ).await?;
                
                        return Ok(ResponseData::new(channel_registry));
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolPostgresqlError {bb8_postgresql_error: error}}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolRedisError {bb8_redis_error: error}}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}