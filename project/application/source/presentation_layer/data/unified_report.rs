use extern_crate::serde::Serialize;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Deserialize;

// It is more correct to use Enam in the context of a unified server response.
// The Struct is used, but not Enam, because there are problems in synchronizing the Enum serialization
// and deserialization rules of the used encoding protocol on both sides.
//
// pub enum UnifiedReport<S>
// {
//     Data {
//         data: Data<S>
//     },
//     ErrorCode {
//         communication_code: &'static str,
//     }
// }

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct UnifiedReport<S>
{
    data: Option<Data<S>>,
    communication_code: Option<&'static str>
}

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
impl<S> UnifiedReport<S>
where
    S: Serialize
{
    pub fn new_without_data(
    ) -> Self {
        return Self {
            data: Some(Data::new(None)),
            communication_code: None
        };
    }

    pub fn new_with_data(
        data: S
    ) -> Self {
        return Self {
            data: Some(Data::new(Some(data))),
            communication_code: None
        };
    }

    pub fn new_with_communication_code(
        communication_code: &'static str
    ) -> Self {
        return Self {
            data: None,
            communication_code: Some(communication_code)
        };
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct UnifiedReport<S>
{
    data: Option<Data<S>>,
    communication_code: Option<String>
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl<S> UnifiedReport<S>
where
    S: Serialize + for<'de> Deserialize<'de>
{
    pub fn new_without_data(
    ) -> Self {
        return Self {
            data: Some(Data::new(None)),
            communication_code: None
        };
    }

    pub fn new_with_data(
        data: S
    ) -> Self {
        return Self {
            data: Some(Data::new(Some(data))),
            communication_code: None
        };
    }

    pub fn new_with_communication_code(
        communication_code: String
    ) -> Self {
        return Self {
            data: None,
            communication_code: Some(communication_code)
        };
    }
}


// It is more correct to use Enam in the context of a unified server response.
// The Struct is used, but not Enam, because there are problems in synchronizing the Enum serialization
// and deserialization rules of the used encoding protocol on both sides.
//
// pub enum Data<S>
// {
//     Empty,
//     Filled {
//         data: S
//     }
// }

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Data<S>
{
    data: Option<S>
}

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
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