use extern_crate::http::header;
use extern_crate::hyper::Body;
use extern_crate::hyper::Request;
use super::action_response_creator::ActionResponseCreator;

pub struct RequestHeaderChecker;

impl RequestHeaderChecker {
    pub fn is_valid<'a>(request: &'a Request<Body>) -> bool {
        let header_map = request.headers();
        if let None = header_map.get(header::USER_AGENT) {
            return false;
        }

        let header_value_content_type = match header_map.get(header::CONTENT_TYPE) {
            Some(header_value_content_type_) => header_value_content_type_,
            None => {
                return false;
            }
        };
        if header_value_content_type.as_bytes() != ActionResponseCreator::HEADER_VALUE_CONTENT_TYPE.as_bytes() {
            return false;
        }

        let header_value_x_content_type_options = match header_map.get(header::X_CONTENT_TYPE_OPTIONS) {
            Some(header_value_x_content_type_options_) => header_value_x_content_type_options_,
            None => {
                return false;
            }
        };
        if header_value_x_content_type_options.as_bytes() != ActionResponseCreator::HEADER_VALUE_X_CONTENT_TYPE_OPTIONS.as_bytes() {
            return false;
        }

        if let None = header_map.get(header::CONTENT_LENGTH) {              // TODO  TODO TODO Как понять, что значение хедера верное? Если значение больше, чем есть на самом желе, то процесс обработки будет ждать дополнительную дату, пока значение из хедера и значение по факту из Боди не сравняется
            return false;
        }

        return true;
    }
}