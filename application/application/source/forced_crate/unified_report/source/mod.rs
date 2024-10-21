use serde::{
    Deserialize,
    Serialize,
};
use bitcode::{
    Encode,
    Decode,
};
#[derive(Serialize, Deserialize, Encode, Decode)]
pub enum UnifiedReport<T, P> {
    Target {
        data: Data<T>,
    },
    Precedent {
        precedent: P,
    },
}
impl<T, P> UnifiedReport<T, P> {
    pub fn target_empty() -> Self {
        return Self::Target {
            data: Data::Empty,
        };
    }
    pub fn target_filled(data: T) -> Self {
        return Self::Target {
            data: Data::Filled {
                data,
            },
        };
    }
    pub fn precedent(precedent: P) -> Self {
        return Self::Precedent {
            precedent,
        };
    }
}
#[derive(Serialize, Deserialize, Encode, Decode)]
pub enum Data<D> {
    Empty,
    Filled {
        data: D,
    },
}