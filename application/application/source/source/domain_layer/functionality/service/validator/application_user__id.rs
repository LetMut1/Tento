use super::Validator;
use crate::domain_layer::data::entity::user::User_Id;
impl Validator<User_Id> {
    pub fn is_valid(user__id: i64) -> bool {
        return user__id >= 0;
    }
}
