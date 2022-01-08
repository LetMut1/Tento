pub mod byte_response_body_wrapper;
pub mod byte_response_creator;
pub mod response_body_wrapper_trait;
pub mod response_creator_trait;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod json_response_body_wrapper;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod json_response_creator;