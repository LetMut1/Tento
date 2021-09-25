
use crate::domain_layer::service::component_validator::_in_context_for::domain_layer::entity::channel::_new_for_context::base_trait::BaseTrait as ChannelComponentValidatorTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::channel::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ChannelDataProviderPostgresql;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::component_validator::_in_context_for::domain_layer::entity::channel::_new_for_context::base::Base as ChannelComponentValidator;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::_component::channel::Channel;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::base::Base as Response;
use std::sync::Arc;

pub struct Base;

impl Base {
    const LIMIT: i8 = 30;

    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<Response, BaseError> 
    {
        let (
            mut channel_name,
            mut requery_channel_name,
            mut limit
        ): (
            String,
            Option<String>,
            i8
        ) = request.into_inner();
        
        if limit <= 0 || limit > Self::LIMIT {
            limit = Self::LIMIT;
        }

        channel_name = String::from_utf8(base64::decode_config(channel_name, base64::URL_SAFE)?)?;
        if !ChannelComponentValidator::is_valid_name(channel_name.as_str()) {
            return Err(BaseError::InvalidArgumentError);
        }
        match requery_channel_name {
            Some(mut requery_channel_name_) => {
                requery_channel_name_ = String::from_utf8(base64::decode_config(requery_channel_name_, base64::URL_SAFE)?)?;
                if !ChannelComponentValidator::is_valid_name(requery_channel_name_.as_str()) {
                    return Err(BaseError::InvalidArgumentError);
                }
                
                requery_channel_name = Some(requery_channel_name_);
            },
            None => {
                requery_channel_name = None;
            }
        }

        let channel_registry: Option<Vec<Channel>> = ChannelDataProviderPostgresql::find_many_by_name(
            &mut *ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, channel_name.as_str(), &requery_channel_name, &(limit as i16)
        )?;

        return Ok(Response::new(channel_registry));
    }
}