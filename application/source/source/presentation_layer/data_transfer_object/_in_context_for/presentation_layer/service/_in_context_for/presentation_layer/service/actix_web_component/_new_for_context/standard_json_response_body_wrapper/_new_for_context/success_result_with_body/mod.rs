use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResultWithBody<'a, S> 
where
    S: Serialize
{
    #[serde(rename = "s")]
    success: bool,
    #[serde(rename = "b")]
    body: &'a S
}

impl<'a, S> SuccessResultWithBody<'a, S>
where
    S: Serialize
{
    pub fn new(
        body: &'a S
    ) -> Self {
        return Self {
            success: true,
            body
        };
    }
}