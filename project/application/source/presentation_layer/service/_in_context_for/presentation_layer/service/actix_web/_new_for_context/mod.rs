pub mod byte_request_data_extractor;
pub mod byte_response_data_wrapper;
pub mod byte_response_creator;
pub mod request_data_extractor_trait;
pub mod response_data_wrapper_trait;
pub mod response_creator_trait;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod json_request_data_extractor;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod json_response_data_wrapper;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod json_response_creator;