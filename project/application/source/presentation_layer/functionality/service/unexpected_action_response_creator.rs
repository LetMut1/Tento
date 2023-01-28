// use extern_crate::http::header;
// use extern_crate::http::HeaderMap;
// use extern_crate::http::HeaderValue;
// use extern_crate::http::Version;
// use extern_crate::hyper::Body;
// use extern_crate::hyper::Response;
// use extern_crate::hyper::StatusCode;
// use std::convert::From;
// use crate::

// pub struct ActionResponseCreator;

// impl ActionResponseCreator {
//     fn create(status_code: StatusCode, data: Option<Vec<u8>>) -> Response<Body> {       // TODO Посмотреть, что за дефолтные ответ. НАстроить необходимое
//         let mut header_map = HeaderMap::new();
//         header_map.append(header::CONTENT_TYPE, HeaderValue::from_static(Self::HEADER_VALUE_CONTENT_TYPE));
//         header_map.append(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static(Self::HEADER_VALUE_X_CONTENT_TYPE_OPTIONS));

//         let mut parts = Response::new(()).into_parts().0;
//         parts.status = status_code;
//         parts.version = Version::HTTP_2;
//         parts.headers = header_map;

//         let body = match data {
//             Some(data_) => Body::from(data_),
//             None => Body::empty()
//         };

//         return Response::from_parts(parts, body);
//     }
// }