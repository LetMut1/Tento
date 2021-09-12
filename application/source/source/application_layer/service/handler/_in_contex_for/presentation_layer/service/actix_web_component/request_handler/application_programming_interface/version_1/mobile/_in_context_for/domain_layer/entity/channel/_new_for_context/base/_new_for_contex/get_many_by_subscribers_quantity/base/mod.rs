
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as DataProviderChannelPostgresql;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::_component::channel::Channel;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::base::Base as Response;
use std::sync::Arc;

pub struct Base;

impl Base {
    const LIMIT_MINIMUM_VALUE: i16 = 300;
    const LIMIT_MAXIMUM_VALUE: i16 = 500;

    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<Response, BaseError> 
    {
        let (
            channel_subscribers_quantity,
            order,
            mut limit
        ): (
            Option<i64>,
            i8,
            i16
        ) = request.into_inner();
        
        if limit < Self::LIMIT_MINIMUM_VALUE || limit > Self::LIMIT_MAXIMUM_VALUE {
            limit = Self::LIMIT_MINIMUM_VALUE;
        }

        if !OrderConventionResolver::can_convert(&order) {
            return Err(BaseError::InvalidArgumentError);
        }

        let channel_registry: Option<Vec<Channel>> = DataProviderChannelPostgresql::find_many_by_subscribers_quantity(
            &mut *ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &channel_subscribers_quantity, &order, &limit
        )?;

        return Ok(Response::new(channel_registry));
    }
}