#[derive(Clone)]
pub struct Header;

impl Header {
    const TYPE: &'static str = "JWT";

    pub const fn new(
    ) -> Self {
        return Self {};
    }

    pub fn get_type<'a>(
        &'a self
    ) -> &'static str {
        return Self::TYPE;
    }
}