use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename = "l")]
    limit: u8,
    #[serde(rename = "o")]
    offset: u64,
}

// impl Query {
//     pub fn get_application_user_email(self) -> String {
//         return self.application_user_email;
//     }
// }