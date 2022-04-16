use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Base as ActionHandlerGetManyByCreatedAt;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Base as ActionHandlerGetManyByIdRegistry;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Base as ActionHandlerGetManyByName;
use crate::application_layer::service::action_handler::_in_contex_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Base as ActionHandlerGetManyBySubscribersQuantity;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::Base as RequestDataGetManyByCreatedAt;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::Base as RequestDataGetManyByIdRegistry;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::Base as RequestDataGetManyByName;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::Base as RequestDataGetManyBySubscribersQuantity;
use crate::presentation_layer::service::_in_context_for::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::endpoint_response::_new_for_context::endpoint_response_creator::EndpointResponseCreator;
use crate::presentation_layer::service::request_header_checker::RequestHeaderChecker;
use crate::presentation_layer::service::response_creator::ResponseCreator;
use hyper::Body;
use hyper::body::HttpBody;
use hyper::Request;
use hyper::Response;
use std::convert::From;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    pub async fn get_many_by_name(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!
        
        match rmp_serde::from_read_ref::<'_, [u8], RequestDataGetManyByName>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerGetManyByName::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error {
                            ErrorAuditor::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                        match json_access_web_token_error {
                                            &JsonAccessWebTokenError::AlreadyExpired => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            ErrorAuditor::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            ErrorAuditor::LogicError {logic_error: _} |
                            ErrorAuditor::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn get_many_by_created_at(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataGetManyByCreatedAt>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerGetManyByCreatedAt::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error {
                            ErrorAuditor::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                        match json_access_web_token_error {
                                            &JsonAccessWebTokenError::AlreadyExpired => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            ErrorAuditor::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            ErrorAuditor::LogicError {logic_error: _} |
                            ErrorAuditor::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
    
    pub async fn get_many_by_subscribers_quantity(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataGetManyBySubscribersQuantity>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerGetManyBySubscribersQuantity::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error {
                            ErrorAuditor::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                        match json_access_web_token_error {
                                            &JsonAccessWebTokenError::AlreadyExpired => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            ErrorAuditor::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            ErrorAuditor::LogicError {logic_error: _} |
                            ErrorAuditor::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }

    pub async fn get_many_by_id_registry(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        if !RequestHeaderChecker::is_valid(&request) {
            return ResponseCreator::create_bad_request();
        }
        
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestDataGetManyByIdRegistry>(bytes.chunk()) {
            Ok(request_data) => {
                match ActionHandlerGetManyByIdRegistry::handle(postgresql_connection_pool, redis_connection_pool, request_data).await {
                    Ok(response_data) => { 
                        match rmp_serde::to_vec(&EndpointResponseCreator::create_with_data(response_data)) {
                            Ok(data) => {
                                return ResponseCreator::create_ok(data);
                            }
                            Err(error) => {
                                log::error!("{}", ErrorAuditor::from(error));
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                    Err(error) => {
                        match error {
                            ErrorAuditor::EntityError {ref entity_error} => {
                                match entity_error {
                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                        match json_access_web_token_error {
                                            &JsonAccessWebTokenError::AlreadyExpired => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                match rmp_serde::to_vec(&EndpointResponseCreator::create_with_error_code(
                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                )) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_ok(data);
                                                    }
                                                    Err(error) => {
                                                        log::error!("{}", ErrorAuditor::from(error));
                                
                                                        return ResponseCreator::create_internal_server_error();
                                                    }
                                                }
                                            }
                                            _ => {
                                                unreachable!("{}", error);
                                            }
                                        }
                                    }
                                    _ => {
                                        unreachable!("{}", error);
                                    }
                                }
                            }
                            ErrorAuditor::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            }
                            ErrorAuditor::LogicError {logic_error: _} |
                            ErrorAuditor::RunTimeError {run_time_error: _} => {
                                log::error!("{}", error);
        
                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                log::error!("{}", ErrorAuditor::from(error));

                return ResponseCreator::create_internal_server_error();
            }
        }
    }
}