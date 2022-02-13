use futures_util::future;
use futures_util::future::Ready;
use hyper::Body;
use hyper::Error;
use hyper::Request;
use hyper::Response;
use std::task::Context;
use std::task::Poll;
use tower_service::Service;

pub struct RequestResolver;

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