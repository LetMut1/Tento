use actix_web::web::Buf;
use actix_web::web::Bytes;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use serde::de::DeserializeOwned;
use super::request_data_extractor_trait::RequestDataExtractorTrait;

pub struct MpRequestDataExtractor;

impl<D> RequestDataExtractorTrait<D> for MpRequestDataExtractor
where
    D: DeserializeOwned
{
    type Error = BaseError;

    fn extract<D_>(
        request_data: Bytes
    ) -> Result<D_, Self::Error>
    where
        D_: DeserializeOwned
    {
        return Ok(rmp_serde::from_read_ref::<'_, [u8], D_>(request_data.bytes())?);
    }
}