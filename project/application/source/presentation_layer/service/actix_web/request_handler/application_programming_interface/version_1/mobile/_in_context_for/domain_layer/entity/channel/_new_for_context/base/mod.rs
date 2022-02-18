use actix_http::body::BoxBody;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Buf;
use actix_web::web::Bytes;
use actix_web::web::Data;
use actix_web::web::Payload;
use actix_web::web::ReqData as RequestData;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_created_at::base::Base as HandlerGetManyByCreatedAt;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_id_registry::base::Base as HandlerGetManyByIdRegistry;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_name::base::Base as HandlerGetManyByName;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::channel::_new_for_context::base::_new_for_contex::get_many_by_subscribers_quantity::base::Base as HandlerGetManyBySubscribersQuantity;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPoolXXXxDELETE;
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
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match RequestData::<JsonAccessWebToken<'static>>::extract(&http_request).await {
                    Ok(_request_data) => {
                        match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                            Ok(bytes) => {
                                match rmp_serde::from_read_ref::<'_, [u8], RequestGetManyByName>(bytes.chunk()) {
                                    Ok(request_data) => {
                                        match HandlerGetManyByName::handle(application_data.into_inner(), request_data) {
                                            Ok(response_data) => { 
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                    }
                                                }
                                            },
                                            Err(ref base_error) => {
                                                match base_error {
                                                    BaseError::EntityError {entity_error: _} => {
                                                        unreachable!("{}", base_error);
                                                    },
                                                    BaseError::InvalidArgumentError => {
                                                        return ResponseCreator::create_bad_requestXXXxDelete();
                                                    },
                                                    BaseError::LogicError {logic_error: _} |
                                                    BaseError::RunTimeError {run_time_error: _} => {
                                                        log::error!("{}", base_error);
                                
                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));
                        
                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn get_many_by_created_at(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match RequestData::<JsonAccessWebToken<'static>>::extract(&http_request).await {
                    Ok(_request_data) => {
                        match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                            Ok(bytes) => {
                                match rmp_serde::from_read_ref::<'_, [u8], RequestGetManyByCreatedAt>(bytes.chunk()) {
                                    Ok(request_data) => {
                                        match HandlerGetManyByCreatedAt::handle(application_data.into_inner(), request_data) {
                                            Ok(response_data) => { 
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                    }
                                                }
                                            },
                                            Err(ref base_error) => {
                                                match base_error {
                                                    BaseError::EntityError {entity_error: _} => {
                                                        unreachable!("{}", base_error);
                                                    },
                                                    BaseError::InvalidArgumentError => {
                                                        return ResponseCreator::create_bad_requestXXXxDelete();
                                                    },
                                                    BaseError::LogicError {logic_error: _} |
                                                    BaseError::RunTimeError {run_time_error: _} => {
                                                        log::error!("{}", base_error);
                                
                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));
                        
                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }
    
    pub async fn get_many_by_subscribers_quantity(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match RequestData::<JsonAccessWebToken<'static>>::extract(&http_request).await {
                    Ok(_request_data) => {
                        match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                            Ok(bytes) => {
                                match rmp_serde::from_read_ref::<'_, [u8], RequestGetManyBySubscribersQuantity>(bytes.chunk()) {
                                    Ok(request_data) => {
                                        match HandlerGetManyBySubscribersQuantity::handle(application_data.into_inner(), request_data) {
                                            Ok(response_data) => { 
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                    }
                                                }
                                            },
                                            Err(ref base_error) => {
                                                match base_error {
                                                    BaseError::EntityError {entity_error: _} => {
                                                        unreachable!("{}", base_error);
                                                    },
                                                    BaseError::InvalidArgumentError => {
                                                        return ResponseCreator::create_bad_requestXXXxDelete();
                                                    },
                                                    BaseError::LogicError {logic_error: _} |
                                                    BaseError::RunTimeError {run_time_error: _} => {
                                                        log::error!("{}", base_error);
                                
                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));
                        
                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }

    pub async fn get_many_by_id_registry(
        http_request: HttpRequest,
        payload: Payload
    ) -> HttpResponse<BoxBody> {
        match Data::<AggregateConnectionPoolXXXxDELETE>::extract(&http_request).await {
            Ok(application_data) => {
                match RequestData::<JsonAccessWebToken<'static>>::extract(&http_request).await {
                    Ok(_request_data) => {
                        match Bytes::from_request(&http_request, &mut payload.into_inner()).await {
                            Ok(bytes) => {
                                match rmp_serde::from_read_ref::<'_, [u8], RequestGetManyByIdRegistry>(bytes.chunk()) {
                                    Ok(request_data) => {
                                        match HandlerGetManyByIdRegistry::handle(application_data.into_inner(), request_data) {
                                            Ok(response_data) => { 
                                                match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_success_with_body(response_data)) {
                                                    Ok(data) => {
                                                        return ResponseCreator::create_okXXXxDelete(data);
                                                    },
                                                    Err(error) => {
                                                        log::error!("{}", BaseError::from(error));
                                
                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                    }
                                                }
                                            },
                                            Err(ref base_error) => {
                                                match base_error {
                                                    BaseError::EntityError {entity_error: _} => {
                                                        unreachable!("{}", base_error);
                                                    },
                                                    BaseError::InvalidArgumentError => {
                                                        return ResponseCreator::create_bad_requestXXXxDelete();
                                                    },
                                                    BaseError::LogicError {logic_error: _} |
                                                    BaseError::RunTimeError {run_time_error: _} => {
                                                        log::error!("{}", base_error);
                                
                                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    Err(error) => {
                                        log::error!("{}", BaseError::from(error));
                        
                                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                                    }
                                }
                            },
                            Err(error) => {
                                log::error!("{}", BaseError::from(error));

                                return ResponseCreator::create_internal_server_errorXXXxDelete();
                            }
                        }
                    },
                    Err(error) => {
                        log::error!("{}", BaseError::from(error));

                        return ResponseCreator::create_internal_server_errorXXXxDelete();
                    }
                }
            },
            Err(error) => {
                log::error!("{}", BaseError::from(error));

                return ResponseCreator::create_internal_server_errorXXXxDelete();
            }
        }
    }
}