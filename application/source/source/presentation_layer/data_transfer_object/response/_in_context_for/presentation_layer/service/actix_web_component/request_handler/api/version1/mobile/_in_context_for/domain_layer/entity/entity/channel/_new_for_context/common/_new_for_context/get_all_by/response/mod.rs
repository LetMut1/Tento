use serde::Serialize;

#[derive(Serialize)]
pub struct Response<S>
where
    S: Serialize 
{
    #[serde(rename = "r")]
    result: Vec<S>
}

impl<S> Response<S>
where
    S: Serialize
{
    pub fn new(result: Vec<S>) -> Self {
        return Self {
            result
        };
    }
}