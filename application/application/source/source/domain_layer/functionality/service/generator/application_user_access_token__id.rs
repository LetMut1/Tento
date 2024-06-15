use super::Generator;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use uuid::Uuid;
impl Generator<ApplicationUserAccessToken_Id> {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}
