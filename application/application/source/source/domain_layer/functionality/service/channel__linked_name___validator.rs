use super::validator::Validator;
use crate::domain_layer::data::entity::channel::Channel_LinkedName;

impl Validator<Channel_LinkedName> {
    pub fn is_valid<'a>(channel_linked_name: &'a Channel_LinkedName) -> bool {
        return true; // TODO;
    }
}
