// use actix_web::web::Bytes;
// use serde::Deserialize;
// use std::error::Error;

// pub trait RequestBodyExtractorTrait {
//     type Error: Error;

//     fn extract<'a, D>(
//         request_body_data: &'a Bytes
//     ) -> Result<D, Self::Error>
//     where
//         D: Deserialize<'a>;              // DeserializeOwned
// }