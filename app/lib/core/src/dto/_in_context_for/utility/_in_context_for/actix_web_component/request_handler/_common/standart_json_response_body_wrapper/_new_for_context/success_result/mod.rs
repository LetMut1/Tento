use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResult<'outer, S> 
where
    S: Serialize
{
    success: bool,
    body: &'outer S
}

impl<'outer, S> SuccessResult<'outer, S>
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