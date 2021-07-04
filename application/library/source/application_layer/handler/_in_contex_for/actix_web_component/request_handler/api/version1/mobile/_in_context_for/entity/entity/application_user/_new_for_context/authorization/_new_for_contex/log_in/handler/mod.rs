use crate::infrastructure_layer::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request;
use crate::infrastructure_layer::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::handler::_new_for_context::result::Result as HandlerResult;
use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_error::ApplicationUserLogInTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::entity_error::EntityError;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as JsonAccessWebTokenBlackListRepository;
use crate::domain_layer::service::_in_context_for::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::domain_layer::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::BaseRepositoryProxy;
use crate::domain_layer::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::domain_layer::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::domain_layer::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<HandlerResult, BaseError> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_log_in_token_device_id, 
            application_user_id, 
            application_user_log_in_token_value
        ) = request.into_inner();

        let application_user_log_in_token_device_id: ApplicationUserLogInTokenDeviceId = ApplicationUserLogInTokenDeviceId::new_from_string(
            application_user_log_in_token_device_id
        )?;

        let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

        if let Some(mut application_user_log_in_token) = ApplicationUserLogInTokenBaseRepository::get_by_application_user_id_and_device_id(
            connection, &ApplicationUserId::new(application_user_id), &application_user_log_in_token_device_id
        )?
        {
            if application_user_log_in_token.get_value().get_value() == application_user_log_in_token_value.as_str() {
                if let Some(existing_json_refresh_web_token) = BaseRepositoryProxy::get_by_application_user_id_and_application_user_log_in_token_device_id(
                    connection, application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
                )? 
                {
                    JsonAccessWebTokenBlackListRepository::create(
                        connection, &JsonAccessWebTokenBlackList::new(existing_json_refresh_web_token.get_json_access_web_token_id())
                    )?;

                    BaseRepositoryProxy::delete(connection, &existing_json_refresh_web_token)?;
                }

                let json_refresh_web_token: JsonRefreshWebToken<'_> =
                JsonRefreshWebToken::new(application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id());
                
                ApplicationUserLogInTokenBaseRepository::delete(connection, &application_user_log_in_token)?;

                BaseRepositoryProxy::create(connection, &json_refresh_web_token)?;

                return Ok(
                    HandlerResult::new(
                        SerializationFormResolver::serialize(&JsonAccessWebToken::new(&json_refresh_web_token)?)?,
                        Encoder::encode(&json_refresh_web_token)?
                    )
                );
            }

            application_user_log_in_token.increment_wrong_enter_tries_quantity();

            if application_user_log_in_token.get_wrong_enter_tries_quantity().get_value() >= ApplicationUserLogInToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                ApplicationUserLogInTokenBaseRepository::delete(connection, &application_user_log_in_token)?;
            }
            
            return Err(BaseError::EntityError(EntityError::ApplicationUserLogInTokenError(ApplicationUserLogInTokenError::InvalidValue)));
        }

        return Err(BaseError::EntityError(EntityError::ApplicationUserLogInTokenError(ApplicationUserLogInTokenError::NotFound)));
    }
}