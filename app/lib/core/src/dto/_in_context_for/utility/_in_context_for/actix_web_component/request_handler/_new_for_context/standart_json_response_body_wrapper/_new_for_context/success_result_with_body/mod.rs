use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResultWithBody<'outer, S> 
where
    S: Serialize
{
    #[serde(rename(serialize = "s"))]
    success: bool,
    #[serde(rename(serialize = "b"))]
    body: &'outer S
}

impl<'outer, S> SuccessResultWithBody<'outer, S>
where 
    S: Serialize
{
    pub fn new(body: &'outer S) -> Self {
        return Self {
            success: true,
            body
        };
    }
}