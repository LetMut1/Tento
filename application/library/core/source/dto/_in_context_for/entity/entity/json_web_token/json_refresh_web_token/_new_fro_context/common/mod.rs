use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Common {
    #[serde(rename = "t")] 
    pub json_access_web_token_id: String,
    #[serde(rename = "a")]
    pub expired_at: String
}

impl<'outer> Common {
    pub fn new(json_refresh_web_token: &'outer JsonRefreshWebToken) -> Self {
        return Self {
            json_access_web_token_id: json_refresh_web_token.get_json_access_web_token_id().get_value().to_string(),
            expired_at: json_refresh_web_token.get_expired_at().get_value().to_rfc3339()
        };
    }
}