use std::error::Error;

pub trait SignatureCreatorTrait {
    type Error: Error;

    fn create<'outer_a>(
        header_and_payload: &'outer_a str
    ) -> Result<String, Self::Error>;

    fn is_valid<'outer_a>(
        header_and_payload: &'outer_a str,
        signature: &'outer_a str
    ) -> Result<bool, Self::Error>;
}