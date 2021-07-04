use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResultWithBody<'outer_a, S> 
where
    S: Serialize
{
    #[serde(rename = "s")]
    success: bool,
    #[serde(rename = "b")]
    body: &'outer_a S
}

impl<'outer_a, S> SuccessResultWithBody<'outer_a, S>
where 
    S: Serialize
{
    pub fn new(body: &'outer_a S) -> Self {
        return Self {
            success: true,
            body
        };
    }
}