use super::Validator;
use crate::domain_layer::data::entity::channel::Channel_Description;
impl Validator<Channel_Description> {
    pub fn is_valid<'a>(channel_description: &'a str) -> bool {
        return channel_description.chars().count() <= Channel_Description::MAXIMUM_LENGTH;
    }
}
