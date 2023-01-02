use extern_crate::serde::Serialize;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Deserialize;

#[derive(Serialize)]
#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[serde(crate = "extern_crate::serde")]
pub struct UnifiedReport<S>
{
    data: Option<Data<S>>,
    communication_code: Option<i64>
}

#[cfg(not(feature = "facilitate_non_automatic_functional_testing"))]
impl<S> UnifiedReport<S>
where
    S: Serialize
{
    pub fn empty() -> Self {
        return Self {
            data: Some(Data::new(None)),
            communication_code: None
        };
    }

    pub fn data(data: S) -> Self {
        return Self {
            data: Some(Data::new(Some(data))),
            communication_code: None
        };
    }

    pub fn communication_code(communication_code: i64) -> Self {
        return Self {
            data: None,
            communication_code: Some(communication_code)
        };
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl<S> UnifiedReport<S>
where
    S: Serialize + for<'de> Deserialize<'de>
{
    pub fn empty() -> Self {
        return Self {
            data: Some(Data::new(None)),
            communication_code: None
        };
    }

    pub fn data(data: S) -> Self {
        return Self {
            data: Some(Data::new(Some(data))),
            communication_code: None
        };
    }

    pub fn communication_code(communication_code: i64) -> Self {
        return Self {
            data: None,
            communication_code: Some(communication_code)
        };
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Data<S>
{
    data: Option<S>
}

#[cfg(not(feature = "facilitate_non_automatic_functional_testing"))]
impl<S> Data<S>
where
    S: Serialize
{
    pub fn new(data: Option<S>) -> Self {
        return Self {
            data
        };
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl<S> Data<S>
where
    S: Serialize + for<'de> Deserialize<'de>
{
    pub fn new(data: Option<S>) -> Self {
        return Self {
            data
        };
    }
}