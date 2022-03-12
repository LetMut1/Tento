use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Base as HandlerGetManyByCreatedAt;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Base as HandlerGetManyByIdRegistry;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Base as HandlerGetManyByName;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Base as HandlerGetManyBySubscribersQuantity;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::Base as RequestGetManyByCreatedAt;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::Base as RequestGetManyByIdRegistry;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::Base as RequestGetManyByName;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::Base as RequestGetManyBySubscribersQuantity;
use crate::presentation_layer::service::response_creator::ResponseCreator;
use crate::presentation_layer::service::response_data_wrapper::ResponseDataWrapper;
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
        _json_access_web_token: JsonAccessWebToken<'_>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!
        
        match rmp_serde::from_read_ref::<'_, [u8], RequestGetManyByName>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerGetManyByName::handle(postgresql_connection_pool, request_data).await {
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
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error: _} => {
                                unreachable!("{}", base_error);
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
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
    }

    pub async fn get_many_by_created_at(
        request: Request<Body>,
        _json_access_web_token: JsonAccessWebToken<'_>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestGetManyByCreatedAt>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerGetManyByCreatedAt::handle(postgresql_connection_pool, request_data).await {
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
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error: _} => {
                                unreachable!("{}", base_error);
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
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
    }
    
    pub async fn get_many_by_subscribers_quantity(
        request: Request<Body>,
        _json_access_web_token: JsonAccessWebToken<'_>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestGetManyBySubscribersQuantity>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerGetManyBySubscribersQuantity::handle(postgresql_connection_pool, request_data).await {
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
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error: _} => {
                                unreachable!("{}", base_error);
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
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
    }

    pub async fn get_many_by_id_registry(
        request: Request<Body>,
        _json_access_web_token: JsonAccessWebToken<'_>,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    ) -> Response<Body> {
        //https://stackoverflow.com/questions/43419974/how-do-i-read-the-entire-body-of-a-tokio-based-hyper-request
        // Обязательно ограничивать количество считываемых байт   https://stackoverflow.com/questions/53142508/how-do-i-apply-a-limit-to-the-number-of-bytes-read-by-futuresstreamconcat2
        // https://github.com/hyperium/hyper/issues/2004
        let bytes = request.into_body().data().await.unwrap().unwrap(); // TODO TODO  TODO  TODO  Неправильный способ !!!!!!!!

        match rmp_serde::from_read_ref::<'_, [u8], RequestGetManyByIdRegistry>(bytes.chunk()) {
            Ok(request_data) => {
                match HandlerGetManyByIdRegistry::handle(postgresql_connection_pool, request_data).await {
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
                    Err(ref base_error) => {
                        match base_error {
                            BaseError::EntityError {entity_error: _} => {
                                unreachable!("{}", base_error);
                            },
                            BaseError::InvalidArgumentError => {
                                return ResponseCreator::create_bad_request();
                            },
                            BaseError::LogicError {logic_error: _} |
                            BaseError::RunTimeError {run_time_error: _} => {
                                log::error!("{}", base_error);
        
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
    }
}