use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use futures_util::future;
use futures_util::future::Ready;
use hyper::Body;
use hyper::Error;
use hyper::Request;
use hyper::Response;
use std::task::Context;
use std::task::Poll;
use tower_service::Service;

pub struct RequestResolver {
    aggregate_connection_pool: AggregateConnectionPool
}

impl RequestResolver {
    pub fn new(
        aggregate_connection_pool: AggregateConnectionPool
    ) -> Self {
        return Self {
            aggregate_connection_pool
        };
    }
}

impl Service<Request<Body>> for RequestResolver {
    type Response = Response<Body>;
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
        request: Request<Body>
    ) -> Self::Future {

        // TODO Роутер

        let acp = self.aggregate_connection_pool.get_postgresql_connection_pool();
        let rsp = Response::builder();

        let uri = request.uri();
        if uri.path() != "/yep" {
            let body = Body::from(Vec::new());
            let rsp = rsp.status(404).body(body).unwrap();
            return future::ok(rsp);
        }

        let body = Body::from(Vec::from(&b"heyo!"[..]));
        let rsp = rsp.status(200).body(body).unwrap();
        future::ok(rsp)
    }
}