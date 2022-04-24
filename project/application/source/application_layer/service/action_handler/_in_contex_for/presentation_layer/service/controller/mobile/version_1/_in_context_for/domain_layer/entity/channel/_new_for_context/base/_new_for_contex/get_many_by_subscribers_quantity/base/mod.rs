
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
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::base::Base as ResponseData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    const LIMIT_MINIMUM_VALUE: i16 = 300;
    const LIMIT_MAXIMUM_VALUE: i16 = 500;

    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAuditor> {
        let (
            json_access_web_token,
            channel_subscribers_quantity,       // TODO // TODO // TODO // TODO // TODO  не нужно ли проверять на >=0 ?
            order,
            mut limit
        ) = request_data.into_inner();

        match redis_connection_pool.get().await {
            Ok(mut redis_pooled_connection) => {
                let _json_access_web_token = Extractor::extract(json_access_web_token.as_str(), &mut *redis_pooled_connection).await?;

                if limit < Self::LIMIT_MINIMUM_VALUE || limit > Self::LIMIT_MAXIMUM_VALUE {
                    limit = Self::LIMIT_MINIMUM_VALUE;
                }
        
                if !OrderConventionResolver::can_convert(&order) {
                    return Err(
                        ErrorAuditor::new(
                            ErrorAggregator::InvalidArgumentError,
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
        
                match postgresql_connection_pool.get().await {
                    Ok(mut postgresql_pooled_connection) => {
                        let channel_registry = ChannelDataProviderPostgresql::per_request_3(
                            &mut *postgresql_pooled_connection, &channel_subscribers_quantity, &order, &limit
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