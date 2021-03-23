use actix_service::Service;
use actix_service::Transform;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use futures::future::ok as FutureOk; 
use futures::future::Ready; 
use super::log_in_resolver::LogInResolver;

pub struct LogInResolverFactory;

impl<S, B> Transform<S> for LogInResolverFactory
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LogInResolver<S, B>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return FutureOk(LogInResolver::new(service));
    }
}