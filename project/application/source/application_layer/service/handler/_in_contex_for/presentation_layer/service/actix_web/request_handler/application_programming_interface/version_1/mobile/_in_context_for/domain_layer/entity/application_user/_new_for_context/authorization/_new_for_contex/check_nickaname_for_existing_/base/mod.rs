use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as Response;
use std::io::Read;
use ureq::Request as UreqRequest;
use ureq::Response as UreqResponse;

pub struct Base;

impl Base {
    pub fn handle(
        request: Request
    ) -> Result<Response, BaseError> {
        let mut data: Vec<u8> = vec![];
        rmp_serde::encode::write(&mut data, &request)?;

        let request_: UreqRequest = ureq::get("http://127.0.0.1:80/v1/m/na/au/cnfe");     // TODO Адрес через конфиг/енв и константы.
        let response_: UreqResponse = request_.send_bytes(&data[..])?;

        data.clear();
        response_.into_reader().read_to_end(&mut data)?;

        let response: Response = rmp_serde::from_read_ref::<'_, [u8], Response>(&data[..])?;

        return Ok(response);
    }
}