use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub trait SignatureCreatorTrait {
    fn create<'outer_a>(header_and_payload: &'outer_a str) -> Result<String, BaseError>;

    fn is_valid<'outer_a>(header_and_payload: &'outer_a str, signature: &'outer_a str) -> Result<bool, BaseError>;
}