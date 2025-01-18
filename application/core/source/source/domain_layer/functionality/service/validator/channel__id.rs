use super::Validator;
use crate::domain_layer::data::entity::channel::Channel_Id;
impl Validator<Channel_Id> {
    pub fn is_valid(channel__id: i64) -> bool {
        return channel__id >= 0;
    }
}
