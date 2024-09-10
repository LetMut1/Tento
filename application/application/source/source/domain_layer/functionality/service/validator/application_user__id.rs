use super::Validator;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
impl Validator<ApplicationUser_Id> {
    pub fn is_valid(application_user__id: i64) -> bool {
        return application_user__id >= 0;
    }
}
