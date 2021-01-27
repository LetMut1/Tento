pub mod core;

use self::core::alg::Alg;
use self::core::typ::Typ;
use std::default::Default;

pub struct Header {
    alg: Alg,
    typ: Typ
}

impl<'a> Header {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn get_alg(&'a self) -> &'a Alg {
        return &self.alg;
    }

    pub fn get_typ(&'a self) -> &'a Typ {
        return &self.typ;
    }
}

impl Default for Header {
    fn default() -> Self {
        return Self {
            alg: Alg::default(),
            typ: Typ::default()
        };
    }
}