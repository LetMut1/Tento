use super::Validator;
use crate::domain_layer::data::entity::channel::Channel_Description;
impl Validator<Channel_Description> {
    pub fn is_valid<'a>(channel__description: &'a str) -> bool {
        return channel__description.chars().count() <= Channel_Description::MAXIMUM_LENGTH;
    }
}
