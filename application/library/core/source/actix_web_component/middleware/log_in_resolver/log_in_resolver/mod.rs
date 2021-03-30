use actix_service::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use actix_web::HttpMessage;
use actix_web::HttpResponse;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::service::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use futures::future::Either;
use futures::future::ok as FutureOk;
use futures::future::Ready;
use std::task::Context;
use std::task::Poll;

pub struct LogInResolver<S, B>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static
{
    service: S,
}

impl<S, B> LogInResolver<S, B>
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

impl<S, B> Service for LogInResolver<S, B>
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
        match service_request.headers().get("X-Auth-Token") {
            Some(header_value) => {
                match header_value.to_str() {
                    Ok(header_value) => {
                        match SerializationFormResolver::deserialize(header_value) {
                            Ok(json_access_web_token) => {
                                if !json_access_web_token.is_expired() {
                                    // TODO BlackList
                                    service_request.extensions_mut().insert::<JsonAccessWebToken<'_>>(json_access_web_token);

                                    return Either::Left(self.service.call(service_request));
                                }
                            },
                            Err(_) => {}
                        };
                    },
                    Err(_) => {}
                };
            },
            None => {}
        };

        return Either::Right(FutureOk(service_request.into_response(HttpResponse::NotFound().finish().into_body())));
    }
}