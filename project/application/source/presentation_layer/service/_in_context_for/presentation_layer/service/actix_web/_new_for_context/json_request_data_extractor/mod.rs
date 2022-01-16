// use actix_web::web::Buf;
// use actix_web::web::Bytes;
// use crate::infrastructure_layer::error::base_error::base_error::BaseError;
// use serde::Deserialize;
// use super::request_body_extractor_trait::RequestDataExtractorTrait;

// pub struct JsonRequestDataExtractor;

// impl RequestDataExtractorTrait for JsonRequestDataExtractor {
//     type Error = BaseError;

//     fn extract<'a, D>(
//         request_body_data: &'a Bytes
//     ) -> Result<D, Self::Error>
//     where
//         D: Deserialize<'a>
//     {
//         return Ok(serde_json::from_slice::<'_, D>(request_body_data.bytes())?);
//     }
// }