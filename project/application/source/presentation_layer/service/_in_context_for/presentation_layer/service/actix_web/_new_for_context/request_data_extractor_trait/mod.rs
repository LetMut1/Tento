use actix_web::web::Bytes;
use serde::de::DeserializeOwned;
use std::error::Error;

pub trait RequestDataExtractorTrait<D>
where
    D: DeserializeOwned
{
    type Error: Error;

    fn extract<D_>(
        request_data: Bytes
    ) -> Result<D_, Self::Error>
    where
        D_: DeserializeOwned;
}