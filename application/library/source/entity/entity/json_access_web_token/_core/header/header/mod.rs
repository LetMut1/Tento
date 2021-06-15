use super::_core::alg::Alg;
use super::_core::typ::Typ;

pub struct Header {
    alg: Alg,
    typ: Typ
}

impl Header {
    pub const fn new() -> Self {
        return Self {
            alg: Alg::new(),
            typ: Typ::new()
        };
    }

    pub fn get_alg<'this>(&'this self) -> &'this Alg {
        return &self.alg;
    }

    pub fn get_typ<'this>(&'this self) -> &'this Typ {
        return &self.typ;
    }
}