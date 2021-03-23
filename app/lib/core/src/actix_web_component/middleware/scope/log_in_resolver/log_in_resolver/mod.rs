use actix_service::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use actix_web::HttpResponse;
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

    fn call(&mut self, request: ServiceRequest) -> Self::Future {

        // TODO
            if request.path() == "/nop" {
                return Either::Left(self.service.call(request))
            } else {
                return Either::Right(FutureOk(request.into_response(HttpResponse::Ok().body("sfsf").into_body())));
            }
        // TODO 
    }
}