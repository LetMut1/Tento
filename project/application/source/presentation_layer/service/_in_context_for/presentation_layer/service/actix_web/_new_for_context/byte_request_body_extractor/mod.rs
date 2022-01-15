// use actix_web::web::Buf;
// use actix_web::web::Bytes;
// use crate::infrastructure_layer::error::base_error::base_error::BaseError;
// use serde::Deserialize;
// use super::request_body_extractor_trait::RequestBodyExtractorTrait;

// pub struct ByteRequestBodyExtractor;

// impl RequestBodyExtractorTrait for ByteRequestBodyExtractor {
//     type Error = BaseError;

//     fn extract<'a, D>(
//         request_body_data: &'a Bytes
//     ) -> Result<D, Self::Error>
//     where
//         D: Deserialize<'a> 
//     {
//         return Ok(rmp_serde::from_read_ref::<'_, [u8], D>(request_body_data.bytes())?);
//     }
// }
