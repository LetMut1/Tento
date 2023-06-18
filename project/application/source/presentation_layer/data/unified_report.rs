use extern_crate::serde::Serialize;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Deserialize;

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub enum UnifiedReport<S>
{
    Data {
        data: Data<S>
    },
    CommunicationCode {
        communication_code: i64
    }
}

#[cfg(not(feature = "facilitate_non_automatic_functional_testing"))]
impl<S> UnifiedReport<S>
where
    S: Serialize
{
    pub fn empty() -> Self {
        return Self::Data { data: Data::Empty };
    }

    pub fn data(data: S) -> Self {
        return Self::Data { data: Data::Filled { data }};
    }

    pub fn communication_code(communication_code: i64) -> Self {
        return Self::CommunicationCode { communication_code };
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl<S> UnifiedReport<S>
where
    S: Serialize + for<'de> Deserialize<'de>
{
    pub fn empty() -> Self {
        return Self::Data { data: Data::Empty };
    }

    pub fn data(data: S) -> Self {
        return Self::Data { data: Data::Filled { data }};
    }

    pub fn communication_code(communication_code: i64) -> Self {
        return Self::CommunicationCode { communication_code };
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub enum Data<S> {
    Empty,
    Filled {
        data: S
    }
}