use super::Validator;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;

impl Validator<ApplicationUserDevice_Id> {
    pub fn is_valid<'a>(application_user_device_id: &'a ApplicationUserDevice_Id) -> bool {
        return true; // TODO
    }
}
