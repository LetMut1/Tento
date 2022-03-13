use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Base as HandlerGetManyByCreatedAt;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Base as HandlerGetManyByIdRegistry;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Base as HandlerGetManyByName;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Base as HandlerGetManyBySubscribersQuantity;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::GetManyByCreatedAt;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::GetManyByIdRegistry;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::GetManyByName;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::GetManyBySubscribersQuantity;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::Base as RequestDataGetManyByCreatedAt;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::Base as RequestDataGetManyByIdRegistry;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::Base as RequestDataGetManyByName;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::Base as RequestDataGetManyBySubscribersQuantity;
use crate::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use crate::presentation_layer::service::response_creator::ResponseCreator;
use crate::presentation_layer::service::response_data_wrapper::ResponseDataWrapper;
use http::header::HeaderName;
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
        match request.headers().get(HeaderName::from_static(Authorization::HEADER_NAME_X_JAWT)) {
            Some(json_access_web_token) => {
                match String::from_utf8(json_access_web_token.as_bytes().to_vec()) {
                    Ok(json_access_web_token_) => {
                        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
                        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
                        // https://github.com/hyperium/hyper/issues/2004
                        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!
                        
                        match rmp_serde::from_read_ref::<'_, [u8], GetManyByName>(bytes.chunk()) {
                            Ok(get_many_by_name) => {
                                let (
                                    channel_name,
                                    requery_channel_name,
                                    limit
                                ) = get_many_by_name.into_inner();

                                match HandlerGetManyByName::handle(
                                    postgresql_connection_pool,
                                    redis_connection_pool,
                                    RequestDataGetManyByName::new(json_access_web_token_, channel_name, requery_channel_name, limit)
                                ).await {
                                    Ok(response_data) => { 
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_ok(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    Err(error) => {
                                        match error {
                                            BaseError::EntityError {ref entity_error} => {
                                                match entity_error {
                                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                                        match json_access_web_token_error {
                                                            &JsonAccessWebTokenError::AlreadyExpired => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_ok(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_error();
                                                                    }
                                                                }
                                                            },
                                                            &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_ok(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_error();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", error);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_request();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", error);
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            None => {
                return ResponseCreator::create_unauthorized();
            }
        }
    }

    pub async fn get_many_by_created_at(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        match request.headers().get(HeaderName::from_static(Authorization::HEADER_NAME_X_JAWT)) {
            Some(json_access_web_token) => {
                match String::from_utf8(json_access_web_token.as_bytes().to_vec()) {
                    Ok(json_access_web_token_) => {
                        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
                        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
                        // https://github.com/hyperium/hyper/issues/2004
                        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

                        match rmp_serde::from_read_ref::<'_, [u8], GetManyByCreatedAt>(bytes.chunk()) {
                            Ok(get_many_by_created_at) => {
                                let (
                                    channel_created_at,
                                    order,
                                    limit
                                ) = get_many_by_created_at.into_inner();

                                match HandlerGetManyByCreatedAt::handle(
                                    postgresql_connection_pool,
                                    redis_connection_pool,
                                    RequestDataGetManyByCreatedAt::new(json_access_web_token_, channel_created_at, order, limit)
                                ).await {
                                    Ok(response_data) => { 
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_ok(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    Err(error) => {
                                        match error {
                                            BaseError::EntityError {ref entity_error} => {
                                                match entity_error {
                                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                                        match json_access_web_token_error {
                                                            &JsonAccessWebTokenError::AlreadyExpired => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_ok(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_error();
                                                                    }
                                                                }
                                                            },
                                                            &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_ok(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_error();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", error);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_request();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", error);
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            None => {
                return ResponseCreator::create_unauthorized();
            }
        }
    }
    
    pub async fn get_many_by_subscribers_quantity(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        match request.headers().get(HeaderName::from_static(Authorization::HEADER_NAME_X_JAWT)) {
            Some(json_access_web_token) => {
                match String::from_utf8(json_access_web_token.as_bytes().to_vec()) {
                    Ok(json_access_web_token_) => {
                        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
                        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
                        // https://github.com/hyperium/hyper/issues/2004
                        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

                        match rmp_serde::from_read_ref::<'_, [u8], GetManyBySubscribersQuantity>(bytes.chunk()) {
                            Ok(get_many_by_subscribers_quantity) => {
                                let (
                                    channel_subscribers_quantity,
                                    order,
                                    limit,
                                ) = get_many_by_subscribers_quantity.into_inner();

                                match HandlerGetManyBySubscribersQuantity::handle(
                                    postgresql_connection_pool,
                                    redis_connection_pool,
                                    RequestDataGetManyBySubscribersQuantity::new(json_access_web_token_, channel_subscribers_quantity, order, limit)
                                ).await {
                                    Ok(response_data) => { 
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_ok(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    Err(error) => {
                                        match error {
                                            BaseError::EntityError {ref entity_error} => {
                                                match entity_error {
                                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                                        match json_access_web_token_error {
                                                            &JsonAccessWebTokenError::AlreadyExpired => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_ok(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_error();
                                                                    }
                                                                }
                                                            },
                                                            &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_ok(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_error();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", error);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_request();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", error);
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            None => {
                return ResponseCreator::create_unauthorized();
            }
        }
    }

    pub async fn get_many_by_id_registry(
        request: Request<Body>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>
    ) -> Response<Body> {
        match request.headers().get(HeaderName::from_static(Authorization::HEADER_NAME_X_JAWT)) {
            Some(json_access_web_token) => {
                match String::from_utf8(json_access_web_token.as_bytes().to_vec()) {
                    Ok(json_access_web_token_) => {
                        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
                        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
                        // https://github.com/hyperium/hyper/issues/2004
                        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

                        match rmp_serde::from_read_ref::<'_, [u8], GetManyByIdRegistry>(bytes.chunk()) {
                            Ok(get_many_by_id_registry) => {
                                match HandlerGetManyByIdRegistry::handle(
                                    postgresql_connection_pool,
                                    redis_connection_pool,
                                    RequestDataGetManyByIdRegistry::new(json_access_web_token_, get_many_by_id_registry.into_inner())
                                ).await {
                                    Ok(response_data) => { 
                                        match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                            Ok(data) => {
                                                return ResponseCreator::create_ok(data);
                                            },
                                            Err(error) => {
                                                log::error!("{}", BaseError::from(error));
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    },
                                    Err(error) => {
                                        match error {
                                            BaseError::EntityError {ref entity_error} => {
                                                match entity_error {
                                                    &EntityError::JsonAccessWebTokenError {ref json_access_web_token_error} => {
                                                        match json_access_web_token_error {
                                                            &JsonAccessWebTokenError::AlreadyExpired => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_ok(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_error();
                                                                    }
                                                                }
                                                            },
                                                            &JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
                                                                    CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_JSON_ACCESS_WEB_TOKEN_BLACK_LIST
                                                                )) {
                                                                    Ok(data) => {
                                                                        return ResponseCreator::create_ok(data);
                                                                    },
                                                                    Err(error) => {
                                                                        log::error!("{}", BaseError::from(error));
                                                
                                                                        return ResponseCreator::create_internal_server_error();
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                unreachable!("{}", error);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        unreachable!("{}", error);
                                                    }
                                                }
                                            },
                                            BaseError::InvalidArgumentError => {
                                                return ResponseCreator::create_bad_request();
                                            },
                                            BaseError::LogicError {logic_error: _} |
                                            BaseError::RunTimeError {run_time_error: _} => {
                                                log::error!("{}", error);
                        
                                                return ResponseCreator::create_internal_server_error();
                                            }
                                        }
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_error();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_error();
                    }
                }
            },
            None => {
                return ResponseCreator::create_unauthorized();
            }
        }
    }
}