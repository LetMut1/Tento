use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResultWithBody<S> 
where
    S: Serialize
{
    #[serde(rename = "s")]
    success: bool,
    #[serde(rename = "b")]
    body: S
}

impl<S> SuccessResultWithBody<S>
where
    S: Serialize
{
    pub fn new(
        body: S
    ) -> Self {
        return Self {
            success: true,
            body
        };
    }
}