use {
    super::Validator,
    crate::infrastructure_layer::{
        data::control_type::Response,
        functionality::service::creator::Creator,
    },
    http::{
        header,
        request::Parts,
    },
};
impl Validator<Parts> {
    pub fn is_valid<'a>(parts: &'a Parts) -> bool {
        let header_value_content_type = match parts.headers.get(header::CONTENT_TYPE) {
            Option::Some(header_value_content_type_) => header_value_content_type_,
            Option::None => {
                return false;
            }
        };
        if *header_value_content_type != Creator::<Response>::HEADER_VALUE_CONTENT_TYPE {
            return false;
        }
        return parts.headers.get(header::CONTENT_LENGTH).is_some(); // TODO  TODO TODO Как понять, что значение хедера верное? Если значение больше, чем есть на самом желе, то процесс обработки будет ждать дополнительную дату, пока значение из хедера и значение по факту из Боди не сравняется
    }
}
