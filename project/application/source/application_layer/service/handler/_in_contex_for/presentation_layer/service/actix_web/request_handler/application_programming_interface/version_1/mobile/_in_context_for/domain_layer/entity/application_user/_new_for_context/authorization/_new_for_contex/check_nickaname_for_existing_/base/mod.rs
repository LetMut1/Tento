use actix_web::http::HeaderMap as ActixWebHeaderMap;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_data_wrapper_trait::_new_for_context::wrapped_response::WrappedResponse;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as Response;
use reqwest::blocking::Client;
use reqwest::blocking::Response as ReqwestResponse;
use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::StatusCode;
use reqwest::Url;
use std::convert::From;
use std::io::Read;
use std::str::FromStr;

pub struct Base;

impl Base {
    pub fn handle<'a>(
        request: Request,
        header_map: &'a ActixWebHeaderMap
    ) -> Result<(Option<WrappedResponse<Response>>, StatusCode), BaseError> {
        let mut header_map_: HeaderMap<HeaderValue> = HeaderMap::new();
        for (header_name, header_value) in header_map.into_iter() {
            header_map_.insert(header_name.clone(), header_value.clone());
        }

        let mut data: Vec<u8> = vec![];
        rmp_serde::encode::write(&mut data, &request)?;

        header_map_.insert(header::CONTENT_LENGTH, HeaderValue::from(data.len()));

        let url: Url = Url::from_str("http://127.0.0.1:80/v1/m/na/au/cnfe").unwrap(); // TODO remove .unwrap()   // TODO Адрес через конфиг/енв и константы.

        let client: Client = Client::builder()
            .default_headers(header_map_)
            .no_brotli()
            .no_deflate()
            .no_gzip()
            .build()?;
        
        let mut response_: ReqwestResponse = client.get(url)
            .body(data)
            .send().unwrap();   // TOOD dlete unwrap

        let response_status_code: u16 = response_.status().as_u16();

        let mut response: Option<WrappedResponse<Response>> = None;
        if response_status_code == StatusCode::OK.as_u16() {
            let mut data: Vec<u8> = vec![];
            response_.read_to_end(&mut data)?;

            response = Some(rmp_serde::from_read_ref::<'_, [u8], WrappedResponse<Response>>(&data[..])?);
        }

        return Ok((response, StatusCode::from_u16(response_status_code).unwrap()));  // TODO delete unwrap() and resolve error 
    }
}