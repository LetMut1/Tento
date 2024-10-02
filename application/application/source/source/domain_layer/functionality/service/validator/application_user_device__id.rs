use super::Validator;
use crate::domain_layer::data::entity::user_device::UserDevice_Id;
impl Validator<UserDevice_Id> {
    pub fn is_valid<'a>(application_user_device__id: &'a str) -> bool {
        return true; // TODO
    }
}
