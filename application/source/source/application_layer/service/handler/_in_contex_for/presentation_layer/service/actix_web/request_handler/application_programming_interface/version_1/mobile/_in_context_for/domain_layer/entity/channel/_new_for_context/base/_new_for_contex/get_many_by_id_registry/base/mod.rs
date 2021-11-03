
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::_component::channel::Channel;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::base::Base as Response;
use std::sync::Arc;

pub struct Base;

impl Base {
    const CHANNEL_ID_REGISTRY_LENGTH_LIMIT: usize = 30;

    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<Response, BaseError> 
    {
        let channel_id_registry: String = request.get_channel_id_registry();
        
        let channel_id_registry_: Vec<i64> = serde_json::from_str::<Vec<i64>>(channel_id_registry.as_str())?;
        if channel_id_registry_.len() == 0 || channel_id_registry_.len() > Self::CHANNEL_ID_REGISTRY_LENGTH_LIMIT {
            return Err(BaseError::InvalidArgumentError);
        }

        let channel_registry: Option<Vec<Channel>> = ChannelDataProviderPostgresql::find_many_by_id_registry(
            &mut *ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &channel_id_registry_
        )?;

        return Ok(Response::new(channel_registry));
    }
}