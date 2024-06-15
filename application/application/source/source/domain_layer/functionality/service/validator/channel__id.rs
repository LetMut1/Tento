use super::Validator;
use crate::domain_layer::data::entity::channel::Channel_Id;
impl Validator<Channel_Id> {
    pub fn is_valid(channel_id: i64) -> bool {
        return channel_id >= 0;
    }
}
