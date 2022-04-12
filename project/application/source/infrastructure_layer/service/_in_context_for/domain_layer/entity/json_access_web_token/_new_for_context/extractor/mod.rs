use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver_trait::ExpirationTimeResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListDataProviderRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use redis::aio::Connection;

pub struct Extractor;

impl Extractor {
    pub async fn extract<'a>(
        json_access_web_token_classic_form: &'a str,
        connection: &'a mut Connection
    ) -> Result<JsonAccessWebToken<'static>, ErrorAggregator> {
        let json_access_web_token = SerializationFormResolver::deserialize(json_access_web_token_classic_form)?;
        if !ExpirationTimeResolver::is_expired(&json_access_web_token)? {
            if !JsonAccessWebTokenBlackListDataProviderRedis::is_exist_by_json_access_token_id(connection, json_access_web_token.get_id()).await? {
                return Ok(json_access_web_token);
            }

            return Err(ErrorAggregator::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::InJsonAccessWebTokenBlackList}});
        }

        return Err(ErrorAggregator::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::AlreadyExpired}});
    }
}