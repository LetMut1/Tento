use super::Validator;
use crate::domain_layer::data::entity::channel::Channel_LinkedName;
impl Validator<Channel_LinkedName> {
    pub fn is_valid<'a>(channel__linked_name: &'a str) -> bool {
        return true; // TODO;
    }
}
