use serde::Serialize;
use super::_component::data::Data;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
pub enum UnifiedReport<S>
where
    S: Serialize
{
    Data {
        data: Data<S>
    },
    ErrorCode {
        #[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
        error_code: &'static str,
        #[cfg(feature="facilitate_non_automatic_functional_testing")]
        error_code: String
    }
}

impl<S> UnifiedReport<S>
where
    S: Serialize
{
    pub fn new_without_data(
    ) -> Self {
        return Self::Data { data: Data::Empty };
    }

    pub fn new_with_data(
        data: S
    ) -> Self {
        return Self::Data { data: Data::Filled { data } };
    }

    pub fn new_with_error_code(
        #[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
        error_code: &'static str,
        #[cfg(feature="facilitate_non_automatic_functional_testing")]
        error_code: String
    ) -> Self {
        return Self::ErrorCode { error_code };
    }
}