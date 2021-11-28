use std::error::Error;

pub trait SignatureCreatorTrait {
    type Error: Error;

    fn create<'a>(
        header: &'a str,
        payload: &'a str
    ) -> Result<String, Self::Error>;

    fn is_valid<'a>(
        header: &'a str,
        payload: &'a str,
        signature: &'a str
    ) -> Result<bool, Self::Error>;
}