use serde::Serialize;
use super::_component::data::Data;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

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
//         user_error_code: &'static str,
//     }
// }

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
#[derive(Serialize)]
pub struct UnifiedReport<S>
{
    data: Option<Data<S>>,
    user_error_code: Option<&'static str>
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
            user_error_code: None
        };
    }

    pub fn new_with_data(
        data: S
    ) -> Self {
        return Self {
            data: Some(Data::new(Some(data))),
            user_error_code: None
        };
    }

    pub fn new_with_error_code(
        error_code: &'static str
    ) -> Self {
        return Self {
            data: None,
            user_error_code: Some(error_code)
        };
    }
}

#[cfg(feature="facilitate_non_automatic_functional_testing")]
#[derive(Serialize, Deserialize)]
pub struct UnifiedReport<S>
{
    data: Option<Data<S>>,
    user_error_code: Option<String>
}

#[cfg(feature="facilitate_non_automatic_functional_testing")]
impl<S> UnifiedReport<S>
where
    S: Serialize + for<'de> Deserialize<'de>
{
    pub fn new_without_data(
    ) -> Self {
        return Self {
            data: Some(Data::new(None)),
            user_error_code: None
        };
    }

    pub fn new_with_data(
        data: S
    ) -> Self {
        return Self {
            data: Some(Data::new(Some(data))),
            user_error_code: None
        };
    }

    pub fn new_with_error_code(
        error_code: String
    ) -> Self {
        return Self {
            data: None,
            user_error_code: Some(error_code)
        };
    }
}