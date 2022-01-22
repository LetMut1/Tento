use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
pub struct WrappedResult<S> 
where
    S: Serialize
{
    success: bool,
    error_code: Option<&'static str>,
    body: Option<S>
}

impl<S> WrappedResult<S>
where
    S: Serialize
{
    pub fn new(
        success: bool,
        error_code: Option<&'static str>,
        body: Option<S>
    ) -> Self {
        return Self {
            success,
            error_code,
            body
        };
    }
}