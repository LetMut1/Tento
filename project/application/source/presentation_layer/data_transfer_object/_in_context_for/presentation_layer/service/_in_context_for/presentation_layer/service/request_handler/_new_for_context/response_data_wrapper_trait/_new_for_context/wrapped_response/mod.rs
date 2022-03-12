use serde::Serialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[derive(Serialize)]
#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
pub struct WrappedResponse<S> 
where
    S: Serialize
{
    success: bool,
    #[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
    error_code: Option<&'static str>,
    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    error_code: Option<String>,
    body: Option<S>
}

impl<S> WrappedResponse<S>
where
    S: Serialize
{
    pub fn new(
        success: bool,
        #[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
        error_code: Option<&'static str>,
        #[cfg(feature="facilitate_non_automatic_functional_testing")]
        error_code: Option<String>,
        body: Option<S>
    ) -> Self {
        return Self {
            success,
            error_code,
            body
        };
    }
}