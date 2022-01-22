use actix_web::body::Body;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Buf;
use actix_web::web::Bytes;
use actix_web::web::Data;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Base as HandlerGetManyByCreatedAt;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Base as HandlerGetManyByIdRegistry;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Base as HandlerGetManyByName;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Base as HandlerGetManyBySubscribersQuantity;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_created_at::base::Base as RequestGetManyByCreatedAt;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_id_registry::base::Base as RequestGetManyByIdRegistry;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_name::base::Base as RequestGetManyByName;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_context::get_many_by_subscribers_quantity::base::Base as RequestGetManyBySubscribersQuantity;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_creator::ResponseCreator;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_data_wrapper::ResponseDataWrapper;
use std::convert::From;

pub struct Base;

impl Base {
    pub async fn get_many_by_name(
        http_request: HttpRequest
    ) -> HttpResponse<Body> {
        match Data::<AggregateConnectionPool>::extract(&http_request).await {
            Ok(data) => {
                match Bytes::extract(&http_request).await {
                    Ok(request_data) => {
                        match rmp_serde:: from_read_ref::<'_, [u8], RequestGetManyByName>(request_data.bytes()) {
                            Ok(request_data_) => {
                                match HandlerGetManyByName::handle(data.into_inner(), request_data_) {
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
    }

    pub async fn get_many_by_created_at(
        http_request: HttpRequest
    ) -> HttpResponse<Body> {
        match Data::<AggregateConnectionPool>::extract(&http_request).await {
            Ok(data) => {
                match Bytes::extract(&http_request).await {
                    Ok(request_data) => {
                        match rmp_serde:: from_read_ref::<'_, [u8], RequestGetManyByCreatedAt>(request_data.bytes()) {
                            Ok(request_data_) => {
                                match HandlerGetManyByCreatedAt::handle(data.into_inner(), request_data_) {
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
    }
    
    pub async fn get_many_by_subscribers_quantity(
        http_request: HttpRequest
    ) -> HttpResponse<Body> {
        match Data::<AggregateConnectionPool>::extract(&http_request).await {
            Ok(data) => {
                match Bytes::extract(&http_request).await {
                    Ok(request_data) => {
                        match rmp_serde:: from_read_ref::<'_, [u8], RequestGetManyBySubscribersQuantity>(request_data.bytes()) {
                            Ok(request_data_) => {
                                match HandlerGetManyBySubscribersQuantity::handle(data.into_inner(), request_data_) {
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
    }

    pub async fn get_many_by_id_registry(
        http_request: HttpRequest
    ) -> HttpResponse<Body> {
        match Data::<AggregateConnectionPool>::extract(&http_request).await {
            Ok(data) => {
                match Bytes::extract(&http_request).await {
                    Ok(request_data) => {
                        match rmp_serde:: from_read_ref::<'_, [u8], RequestGetManyByIdRegistry>(request_data.bytes()) {
                            Ok(request_data_) => {
                                match HandlerGetManyByIdRegistry::handle(data.into_inner(), request_data_) {
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
    }
}