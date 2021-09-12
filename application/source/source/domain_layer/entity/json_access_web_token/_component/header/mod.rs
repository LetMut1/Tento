#[derive(Clone)]
pub struct Header;

impl Header {
    const TYPE: &'static str = "JWT";

    pub const fn new(
    ) -> Self {
        return Self {};
    }

    pub fn get_type<'this>(
        &'this self
    ) -> &'static str {
        return Self::TYPE;
    }
}