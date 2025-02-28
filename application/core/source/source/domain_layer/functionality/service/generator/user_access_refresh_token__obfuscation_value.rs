use {
    super::Generator,
    crate::domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken_ObfuscationValue,
    uuid::Uuid,
};
impl Generator<UserAccessRefreshToken_ObfuscationValue> {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}
