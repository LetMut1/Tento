// use actix_service::Service;
// use actix_web::dev::ServiceRequest;
// use actix_web::dev::ServiceResponse;
// use actix_web::Error;
// use actix_web::HttpMessage;
// use actix_web::web::Data;
// use crate::application_layer::service::handler::_in_contex_for::presentation_layer::service::actix_web::middleware::authentication_resolver::authentication_resolver::_new_for_contex::call::base::Base as HandlerBase;
// use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
// use crate::domain_layer::error::entity_error::entity_error::EntityError;
// use crate::domain_layer::service::_in_context_for::domain_layer::error::_new_for_context::communication_code_storage::CommunicationCodeStorage;
// use crate::infrastructure_layer::error::base_error::_component::logic_error::LogicError;
// use crate::infrastructure_layer::error::base_error::base_error::BaseError;
// use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
// use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_creator::ResponseCreator;
// use crate::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_data_wrapper::ResponseDataWrapper;
// use futures::future;
// use futures::future::Either;
// use futures::future::Ready;
// use std::task::Context;
// use std::task::Poll;

// pub struct AuthenticationResolver<S, B>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static
// {
//     service: S
// }

// impl<S, B> AuthenticationResolver<S, B>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static
// {
//     pub fn new(
//         service: S
//     ) -> Self {
//         return Self {
//             service
//         };
//     }
// }

// impl<S, B> Service<ServiceRequest> for AuthenticationResolver<S, B>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

//     fn poll_ready<'a, 'b>(
//         &'a mut self,
//         context: &'b mut Context
//     ) -> Poll<Result<(), Self::Error>> {
//         return self.service.poll_ready(context);
//     }

//     fn call<'a>(
//         &'a mut self,
//         service_request: ServiceRequest
//     ) -> Self::Future {
//         if let Some(data) = service_request.app_data::<Data<AggregateConnectionPool>>() {
//             match HandlerBase::handle(data.clone().into_inner(), service_request.headers()) { // TODO В такой реализации При инъекции в РекуестХэндлер этот объкт будет склонирован. А нужно передать его самого. Актикс сейчас не позволякет это сделать. Проверить позже
//                 Ok(json_access_web_token) => {
//                     service_request.extensions_mut().insert(json_access_web_token);

//                     return Either::Left(self.service.call(service_request));
//                 },
//                 Err(ref base_error) => {
//                     match base_error {
//                         BaseError::EntityError {entity_error} => {
//                             match entity_error {
//                                 EntityError::JsonAccessWebTokenError {json_access_web_token_error} => {
//                                     match json_access_web_token_error {
//                                         JsonAccessWebTokenError::AlreadyExpired => {
//                                             match rmp_serde::to_vec(&ResponseDataWrapper::wrap_for_fail(
//                                                 CommunicationCodeStorage::ENTITY_JSON_ACCESS_WEB_TOKEN_ALREADY_EXPIRED
//                                             )) {
//                                                 Ok(data) => {
//                                                     return Either::Right(future::ok(service_request.into_response(ResponseCreator::create_ok(data).into_body())));
//                                                 },
//                                                 Err(error) => {
//                                                     log::error!("{}", BaseError::from(error));
                            
//                                                     return Either::Right(future::ok(service_request.into_response(ResponseCreator::create_internal_server_error().into_body())));
//                                                 }
//                                             }
//                                         },
//                                         JsonAccessWebTokenError::InJsonAccessWebTokenBlackList |
//                                         JsonAccessWebTokenError::NotFound => {
//                                             return Either::Right(future::ok(service_request.into_response(ResponseCreator::create_unauthorized().into_body())));
//                                         },
//                                         _ => {
//                                             unreachable!("{}", base_error);
//                                         }
//                                     }
//                                 },
//                                 _ => {
//                                     unreachable!("{}", base_error);
//                                 }
//                             }
//                         },
//                         BaseError::InvalidArgumentError => {
//                             return Either::Right(future::ok(service_request.into_response(ResponseCreator::create_bad_request().into_body())));
//                         },
//                         BaseError::LogicError {logic_error: _} |
//                         BaseError::RunTimeError {run_time_error: _} => {
//                             log::error!("{}", base_error);

//                             return Either::Right(future::ok(service_request.into_response(ResponseCreator::create_internal_server_error().into_body())));
//                         }
//                     }
//                 }
//             }
//         }

//         log::error!("{}", BaseError::LogicError {logic_error: LogicError::new(true, "Aggregate_connection_pool must exist in application state.")});

//         return Either::Right(future::ok(service_request.into_response(ResponseCreator::create_internal_server_error().into_body())));
//     }
// }