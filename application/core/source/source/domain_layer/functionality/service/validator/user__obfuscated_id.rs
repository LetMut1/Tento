use {
    super::Validator,
    crate::domain_layer::data::entity::user::User_ObfuscatedId,
};
impl Validator<User_ObfuscatedId> {
    pub fn is_valid(user__obfuscated_id: i64) -> bool {
        return user__obfuscated_id >= 0;
    }
}
