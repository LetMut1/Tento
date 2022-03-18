use http::Request;
use hyper::Body;

pub struct Base {
    proxied_request: Request<Body>
}

impl Base {
    pub fn new(
        proxied_request: Request<Body>
    ) -> Self {
        return Self {
            proxied_request
        };
    }

    pub fn into_inner(
        self
    ) -> Request<Body> {
        return self.proxied_request;
    }
}