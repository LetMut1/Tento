use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResultWithBody<S> 
where
    S: Serialize
{
    success: bool,
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