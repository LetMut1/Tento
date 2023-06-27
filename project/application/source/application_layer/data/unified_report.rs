use extern_crate::serde::Serialize;

#[cfg(feature = "manual_testing")]
use extern_crate::serde::Deserialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub enum UnifiedReport<T, P> {
    Target {
        data: Data<T>,
    },
    Precedent {
        precedent: P,
    },
}

#[cfg(not(feature = "manual_testing"))]
impl<T, P> UnifiedReport<T, P>
where
    T: Serialize,
    P: Serialize,
{
    pub fn empty() -> Self {
        return Self::Target {
            data: Data::Empty,
        };
    }

    pub fn filled(data: T) -> Self {
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

#[cfg(feature = "manual_testing")]
impl<T, P> UnifiedReport<T, P>
where
    T: Serialize + for<'de> Deserialize<'de>,
    P: Serialize + for<'de> Deserialize<'de>,
{
    pub fn empty() -> Self {
        return Self::Target {
            data: Data::Empty,
        };
    }

    pub fn filled(data: T) -> Self {
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

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub enum Data<D> {
    Empty,
    Filled {
        data: D,
    },
}
