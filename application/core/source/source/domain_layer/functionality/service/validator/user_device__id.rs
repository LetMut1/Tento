use {
    super::Validator,
    crate::domain_layer::data::entity::user_device::UserDevice_Id,
};
impl Validator<UserDevice_Id> {
    pub fn is_valid<'a>(user_device__id: &'a str) -> bool {
        return true; // TODO
    }
}
