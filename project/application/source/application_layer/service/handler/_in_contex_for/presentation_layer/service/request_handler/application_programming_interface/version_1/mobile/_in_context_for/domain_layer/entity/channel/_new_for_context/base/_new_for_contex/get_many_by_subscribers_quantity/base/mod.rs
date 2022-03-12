
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::base::Base as ResponseData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    const LIMIT_MINIMUM_VALUE: i16 = 300;
    const LIMIT_MAXIMUM_VALUE: i16 = 500;

    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        request_data: RequestData
    ) -> Result<ResponseData, BaseError> {
        let (
            channel_subscribers_quantity,
            order,
            mut limit
        ) = request_data.into_inner();
        
        if limit < Self::LIMIT_MINIMUM_VALUE || limit > Self::LIMIT_MAXIMUM_VALUE {
            limit = Self::LIMIT_MINIMUM_VALUE;
        }

        if !OrderConventionResolver::can_convert(&order) {
            return Err(BaseError::InvalidArgumentError);
        }

        let channel_registry = ChannelDataProviderPostgresql::per_request_3(
            &mut *postgresql_connection_pool.get().await?, &channel_subscribers_quantity, &order, &limit
        ).await?;

        return Ok(ResponseData::new(channel_registry));
    }
}