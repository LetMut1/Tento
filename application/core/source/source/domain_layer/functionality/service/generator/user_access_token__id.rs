use {
    super::Generator,
    crate::domain_layer::data::entity::user_access_token::UserAccessToken_Id,
    uuid::Uuid,
};
impl Generator<UserAccessToken_Id> {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}
