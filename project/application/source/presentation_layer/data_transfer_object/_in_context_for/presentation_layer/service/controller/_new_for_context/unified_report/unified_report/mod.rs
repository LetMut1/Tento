use serde::Serialize;
use super::_component::data::Data;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
#[derive(Serialize)]
pub enum UnifiedReport<S>
{
    Data {
        data: Data<S>
    },
    ErrorCode {
        error_code: &'static str,
    }
}

impl<S> UnifiedReport<S>
{
    pub fn new_without_data(
    ) -> Self {
        return Self::Data { data: Data::Empty };
    }
}

#[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
impl<S> UnifiedReport<S>
where
    S: Serialize
{
    pub fn new_with_data(
        data: S
    ) -> Self {
        return Self::Data { data: Data::Filled { data } };
    }

    pub fn new_with_error_code(
        error_code: &'static str,
    ) -> Self {
        return Self::ErrorCode { error_code };
    }
}

#[cfg(feature="facilitate_non_automatic_functional_testing")]
#[derive(Serialize, Deserialize)]
pub enum UnifiedReport<S>
{
    Data {
        data: Data<S>
    },
    ErrorCode {
        error_code: String
    }
}

#[cfg(feature="facilitate_non_automatic_functional_testing")]
impl<S> UnifiedReport<S>
where
    S: Serialize + for<'de> Deserialize<'de>
{
    pub fn new_with_data(
        data: S
    ) -> Self {
        return Self::Data { data: Data::Filled { data } };
    }

    pub fn new_with_error_code(
        error_code: String
    ) -> Self {
        return Self::ErrorCode { error_code };
    }
}