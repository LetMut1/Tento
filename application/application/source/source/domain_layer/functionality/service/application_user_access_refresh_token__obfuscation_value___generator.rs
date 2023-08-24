use super::generator::Generator;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use uuid::Uuid;

impl Generator<ApplicationUserAccessRefreshToken_ObfuscationValue> {
    pub fn generate() -> ApplicationUserAccessRefreshToken_ObfuscationValue {
        return ApplicationUserAccessRefreshToken_ObfuscationValue(Uuid::new_v4().to_string());
    }
}
