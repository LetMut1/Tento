use actix_service::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::middleware::authentication_resolver::authentication_resolver::_new_for_contex::call::base::Base as HandlerBase;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::json_response_creator::JsonResponseCreator;
use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_creator_trait::ResponseCreatorTrait;
use futures::future;
use futures::future::Either;
use futures::future::Ready;
use std::task::Context;
use std::task::Poll;

pub struct AuthenticationResolver_<S, B>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static
{
    service: S
}

impl<S, B> AuthenticationResolver_<S, B>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static
{
    pub fn new(
        service: S
    ) -> Self {
        return Self {
            service
        };
    }
}

impl<S, B> Service for AuthenticationResolver_<S, B>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready<'a, 'b>(
        &'a mut self,
        context: &'b mut Context
    ) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(context);
    }

    fn call<'a>(
        &'a mut self,
        service_request: ServiceRequest
    ) -> Self::Future {
        if let Err(ref base_error) = HandlerBase::handle(&service_request) {
            match base_error {
                BaseError::EntityError {entity_error} => {
                    match entity_error {
                        EntityError::JsonAccessWebTokenError {json_access_web_token_error} => {
                            match json_access_web_token_error {
                                JsonAccessWebTokenError::AlreadyExpired => {
                                    match JsonResponseCreator::wrap_for_fail_and_create_ok(
                                        CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                    ) {
                                        Ok(http_response) => {
                                            return Either::Right(future::ok(service_request.into_response(http_response.into_body())));
                                        },
                                        Err(base_error) => {
                                            log::error!("{}", base_error);
                    
                                            return Either::Right(future::ok(service_request.into_response(JsonResponseCreator::create_internal_server_error().into_body())));
                                        }
                                    }
                                },
                                JsonAccessWebTokenError::InJsonAccessWebTokenBlackList |
                                JsonAccessWebTokenError::NotFound => {
                                    return Either::Right(future::ok(service_request.into_response(JsonResponseCreator::create_unauthorized().into_body())));
                                },
                                _ => {
                                    unreachable!("{}", base_error);
                                }
                            }
                        },
                        _ => {
                            unreachable!("{}", base_error);
                        }
                    }
                },
                BaseError::InvalidArgumentError => {
                    return Either::Right(future::ok(service_request.into_response(JsonResponseCreator::create_bad_request().into_body())));
                },
                BaseError::LogicError {logic_error: _} |
                BaseError::RunTimeError {run_time_error: _} => {
                    log::error!("{}", base_error);

                    return Either::Right(future::ok(service_request.into_response(JsonResponseCreator::create_internal_server_error().into_body())));
                }
            }
        }

        return Either::Left(self.service.call(service_request));
    }
}