use actix_web::http::header::HeaderMap;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenBlackListDataProviderRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver_trait::ExpirationTimeResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListDataProviderRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPoolXXXxDELETE;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractorXXXxDelete;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub const HEADER_NAME_X_JAWT: &'static str = "X-Jawt";  // TODO эту константу убрать вообщ в другой файл, а не транслировать

    pub fn handle<'a>(
        aggregate_connection_pool: Arc<AggregateConnectionPoolXXXxDELETE>,
        header_map: &'a HeaderMap
    ) -> Result<JsonAccessWebToken<'static>, BaseError> {
        if let Some(x_jawt_header_value) = header_map.get(Self::HEADER_NAME_X_JAWT) {
            if let Ok(x_jawt) = x_jawt_header_value.to_str() {
                let json_access_web_token = SerializationFormResolver::deserialize(x_jawt)?;
                if !ExpirationTimeResolver::is_expired(&json_access_web_token)? {
                    if !JsonAccessWebTokenBlackListDataProviderRedis::is_exist_by_json_access_token_id(
                        &mut *ConnectionExtractorXXXxDelete::get_redis_connection(&aggregate_connection_pool)?, json_access_web_token.get_id()
                    )? {
                        return Ok(json_access_web_token);
                    }

                    return Err(BaseError::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::InJsonAccessWebTokenBlackList}});
                }

                return Err(BaseError::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::AlreadyExpired}});
            }
        }

        return Err(BaseError::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::NotFound}});
    }
}