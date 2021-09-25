use actix_service::Service;
use actix_service::Transform;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::Error;
use crate::presentation_layer::service::actix_web_component::middleware::service::authentication_resolver::AuthenticationResolver;
use futures::future::ok as FutureOk; 
use futures::future::Ready; 

pub struct AuthenticationResolverFactory;

impl<S, B> Transform<S> for AuthenticationResolverFactory
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationResolver<S, B>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform<'a>(
        &'a self,
        service: S
    ) -> Self::Future {
        return FutureOk(AuthenticationResolver::new(service));
    }
}