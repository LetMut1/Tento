use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::extractor::Extractor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::service::date_time_resolver::DateTimeResolver;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::base::Base as ResponseData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    const LIMIT: i8 = 30;

    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, BaseError> {
        let (
            json_access_web_token,
            channel_created_at,
            order,
            mut limit
        ) = request_data.into_inner();

        let _json_access_web_token = Extractor::extract(json_access_web_token.as_str(), &mut *redis_connection_pool.get().await?).await?;

        if let Some(ref channel_created_at_) = channel_created_at {
            if !DateTimeResolver::is_valid_timestamp(channel_created_at_.as_str()) {
                return Err(BaseError::InvalidArgumentError);
            }
        }

        if !OrderConventionResolver::can_convert(&order) {
            return Err(BaseError::InvalidArgumentError);
        }

        if limit <= 0 || limit > Self::LIMIT {
            limit = Self::LIMIT;
        }

        let channel_registry = ChannelDataProviderPostgresql::per_request_2(
            &mut *postgresql_connection_pool.get().await?, &channel_created_at, &order, &(limit as i16)
        ).await?;

        return Ok(ResponseData::new(channel_registry));
    }
}