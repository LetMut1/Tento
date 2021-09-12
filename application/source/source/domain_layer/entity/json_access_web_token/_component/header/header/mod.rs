#[derive(Clone)]
pub struct Header;

impl Header {
    const ALG: &'static str = "HS512";
    const TYP: &'static str = "JWT";

    pub const fn new() -> Self {
        return Self {};
    }

    pub fn get_alg<'this>(&'this self) -> &'static str  {
        return Self::ALG;
    }

    pub fn get_typ<'this>(&'this self) -> &'static str {
        return Self::TYP;
    }
}