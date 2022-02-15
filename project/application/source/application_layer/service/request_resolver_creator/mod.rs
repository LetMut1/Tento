use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use futures_util::future;
use futures_util::future::Ready;
use std::io::Error;
use std::task::Context;
use std::task::Poll;
use super::request_resolver::RequestResolver;
use tower_service::Service;

pub struct RequestResolverCreator {
    aggregate_connection_pool: AggregateConnectionPool
}

impl RequestResolverCreator {
    pub fn new(
        aggregate_connection_pool: AggregateConnectionPool
    ) -> Self {
        return Self {
            aggregate_connection_pool
        };
    }
}

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
        let request_resolver = RequestResolver::new(self.aggregate_connection_pool.clone());

        return future::ok(request_resolver);
    }
}