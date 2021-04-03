use actix_service::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::json_web_token::json_access_web_token::_new_for_context::json_access_web_token_error_kind::JsonAccessWebTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::handler::_in_contex_for::actix_web_component::middleware::authentication_resolver::authentication_resolver::_new_for_contex::call::handler::Handler as CallHandler;
use crate::utility::_in_context_for::actix_web_component::_new_for_context::standard_json_response_body_wrapper::StandardJsonResponseBodyWrapper;
use crate::utility::_in_context_for::actix_web_component::_new_for_context::standard_response_creator::StandardResponseCreator;
use futures::future::Either;
use futures::future::ok as FutureOk;
use futures::future::Ready;
use std::task::Context;
use std::task::Poll;

pub struct AuthenticationResolver<S, B>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static
{
    service: S,
}

impl<S, B> AuthenticationResolver<S, B>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static
{
    pub fn new(service: S) -> Self {
        return Self {
            service
        };
    }
}

impl<S, B> Service for AuthenticationResolver<S, B>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, context: &mut Context) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(context);
    }

    fn call(&mut self, service_request: ServiceRequest) -> Self::Future {
        if let Err(ref main_error_kind) = CallHandler::handle(&service_request) {
            match main_error_kind {
                MainErrorKind::EntityErrorKind(ref entity_error_kind) => {
                    match entity_error_kind {
                        EntityErrorKind::JsonAccessWebTokenErrorKind(ref json_access_web_token_error_kind) => {
                            match json_access_web_token_error_kind {
                                JsonAccessWebTokenErrorKind::AlreadyExpired => {
                                    return Either::Right(FutureOk(service_request.into_response(StandardResponseCreator::create_ok(StandardJsonResponseBodyWrapper::wrap_for_fail_with_code("enjsacweto03")).into_body())));
                                },
                                JsonAccessWebTokenErrorKind::InJsonAccessWebTokenBlackList => {
                                    return Either::Right(FutureOk(service_request.into_response(StandardResponseCreator::create_unauthorized().into_body())));
                                },
                                _ => {
                                    return Either::Right(FutureOk(service_request.into_response(StandardResponseCreator::create_internal_server_error().into_body())));
                                }
                            }
                        },
                        _ => {
                            return Either::Right(FutureOk(service_request.into_response(StandardResponseCreator::create_internal_server_error().into_body())));
                        }
                    }
                },
                MainErrorKind::InvalidArgumentError => {
                    return Either::Right(FutureOk(service_request.into_response(StandardResponseCreator::create_bad_request().into_body())));
                },
                _ => {
                    return Either::Right(FutureOk(service_request.into_response(StandardResponseCreator::create_internal_server_error().into_body())));
                }
            }
        }

        return Either::Left(self.service.call(service_request));
    }
}