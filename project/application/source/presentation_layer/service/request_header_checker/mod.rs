use http::header;
use hyper::Body;
use hyper::Request;
use super::response_creator::ResponseCreator;

pub struct RequestHeaderChecker;

impl RequestHeaderChecker {
    pub fn is_valid<'a>(
        request: &'a Request<Body>
    ) -> bool {
        let header_map = request.headers();
        match header_map.get(header::CONTENT_TYPE) {
            Some(header_value) => {
                if *header_value != ResponseCreator::HEADER_VALUE_CONTENT_TYPE {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
        match header_map.get(header::X_CONTENT_TYPE_OPTIONS) {
            Some(header_value) => {
                if *header_value != ResponseCreator::HEADER_VALUE_X_CONTENT_TYPE_OPTIONS {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
        if let None = header_map.get(header::CONTENT_LENGTH) {              // TODO  TODO TODO Как понять, что значение хедера верное? Если значение больше, чем есть на самом желе, то процесс обработки будет ждать дополнительную дату, пока значение из хедера и значение по факту из Боди не сравняется
            return false;
        }

        return true;
    }
}