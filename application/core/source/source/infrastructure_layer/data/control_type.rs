use bytes::Bytes;
use http_body_util::Full;
use hyper::{
    body::Incoming,
    Request as HyperRequest,
    Response as HyperResponse,
};
pub type Request = HyperRequest<Incoming>;
pub type Response = HyperResponse<Full<Bytes>>;
