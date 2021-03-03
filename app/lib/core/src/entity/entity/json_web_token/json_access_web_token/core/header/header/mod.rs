use super::core::alg::Alg;
use super::core::typ::Typ;

pub struct Header {
    alg: Alg,
    typ: Typ
}

impl<'this> Header {
    pub const fn new() -> Self {
        return Self {
            alg: Alg::new(),
            typ: Typ::new()
        };
    }

    pub fn get_alg(&'this self) -> &'this Alg {
        return &self.alg;
    }

    pub fn get_typ(&'this self) -> &'this Typ {
        return &self.typ;
    }
}