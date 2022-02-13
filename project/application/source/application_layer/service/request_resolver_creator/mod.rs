use futures_util::future;
use futures_util::future::Ready;
use std::io::Error;
use std::task::Context;
use std::task::Poll;
use super::request_resolver::RequestResolver;
use tower_service::Service;

pub struct RequestResolverCreator;

impl<T> Service<T> for RequestResolverCreator {
    type Response = RequestResolver;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready<'a>(
        &'a mut self,
        _context: &'a mut Context<'_>
    ) -> Poll<Result<(), Self::Error>> {
        return Poll::Ready(Ok(()));
    }

    fn call<'a>(
        &'a mut self,
        _: T
    ) -> Self::Future {
        return future::ok(RequestResolver);
    }
}