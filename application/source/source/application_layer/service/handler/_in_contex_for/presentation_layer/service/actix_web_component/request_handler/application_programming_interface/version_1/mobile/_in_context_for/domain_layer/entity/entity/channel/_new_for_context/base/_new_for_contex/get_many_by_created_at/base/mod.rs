
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as DataProviderChannelPostgresql;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::order_convention_resolver::OrderConventionResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::date_time_resolver::DateTimeResolver;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::_component::channel::Channel;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::base::Base as Response;
use std::sync::Arc;

pub struct Base;

impl Base {
    const LIMIT: u8 = 30;

    pub fn handle<'outer_a>(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<Response, BaseError> 
    {
        let (
            channel_created_at,
            requery_channel_created_at,
            order,
            mut limit
        ): (
            String,
            Option<String>,
            u8,
            u8
        ) = request.into_inner();
        
        if limit == 0 || limit > Self::LIMIT {
            limit = Self::LIMIT;
        }

        if !OrderConventionResolver::can_convert(order) || !DateTimeResolver::is_valid_timestamp(&channel_created_at) {
            return Err(BaseError::InvalidArgumentError);
        }
        if let Some(ref requery_channel_created_at_) = requery_channel_created_at {
            if !DateTimeResolver::is_valid_timestamp(requery_channel_created_at_.as_str()) {
                return Err(BaseError::InvalidArgumentError);
            }
        }

        let channel_registry: Option<Vec<Channel>> = DataProviderChannelPostgresql::get_many_by_created_at(
            &mut *ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, channel_created_at.as_str(), &requery_channel_created_at, order, limit
        )?;

        return Ok(Response::new(channel_registry));
    }
}