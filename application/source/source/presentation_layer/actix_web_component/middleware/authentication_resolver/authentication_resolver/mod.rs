use actix_service::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use crate::application_layer::service::handler::_in_contex_for::presentation_layer::actix_web_component::middleware::authentication_resolver::authentication_resolver::_new_for_contex::call::handler::Handler as CallHandler;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::base_error::_core::entity_error::entity_error::EntityError;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
use crate::presentation_layer::service::_in_context_for::presentation_layer::actix_web_component::_new_for_context::standard_response_creator::StandardResponseCreator;
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

    fn poll_ready<'this, 'outer_a>(&'this mut self, context: &'outer_a mut Context) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(context);
    }

    fn call<'this>(&'this mut self, service_request: ServiceRequest) -> Self::Future {
        if let Err(ref base_error) = CallHandler::handle(&service_request) {
            match base_error {
                BaseError::EntityError(entity_error) => {
                    match entity_error {
                        EntityError::JsonAccessWebTokenError(json_access_web_token_error) => {
                            match json_access_web_token_error {
                                JsonAccessWebTokenError::AlreadyExpired => {
                                    return Either::Right(FutureOk(service_request.into_response(
                                        StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                            CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
                                        ).into_body()
                                    )));
                                },
                                JsonAccessWebTokenError::InJsonAccessWebTokenBlackList => {
                                    return Either::Right(FutureOk(service_request.into_response(
                                        StandardResponseCreator::wrap_for_fail_with_code_and_create_ok(
                                            CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_IN_BLACK_LIST
                                        ).into_body()
                                    )));
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
                    return Either::Right(FutureOk(service_request.into_response(StandardResponseCreator::create_bad_request().into_body())));
                },
                BaseError::LogicError(_) | BaseError::RunTimeError(_) => {
                    log::error!("{}", base_error);

                    return Either::Right(FutureOk(service_request.into_response(StandardResponseCreator::create_internal_server_error().into_body())));
                }
            }
        }

        return Either::Left(self.service.call(service_request));
    }
}