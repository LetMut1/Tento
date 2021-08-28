use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "l")]
    limit: u8
}

impl Base {
    pub fn into_inner(self) -> (String, u8) {
        return (
            self.name,
            self.limit
        );
    }
}