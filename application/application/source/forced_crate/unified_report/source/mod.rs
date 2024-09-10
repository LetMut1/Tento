use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub enum UnifiedReport<T, P> {
    Target {
        data: Data<T>,
    },
    Precedent {
        precedent: P,
    },
}
impl<T, P> UnifiedReport<T, P>
where
    T: Serialize + for<'de> Deserialize<'de>,
    P: Serialize + for<'de> Deserialize<'de>,
{
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
#[derive(Serialize, Deserialize)]
pub enum Data<D> {
    Empty,
    Filled {
        data: D,
    },
}
